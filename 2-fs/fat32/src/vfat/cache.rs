use std::collections::HashMap;
use std::{
    fmt,
    io::{self, Write},
};

use traits::BlockDevice;

#[derive(Debug)]
struct CacheEntry {
    data: Vec<u8>,
    dirty: bool,
}

impl CacheEntry {
    pub fn new(data: Vec<u8>) -> Self {
        CacheEntry { dirty: false, data }
    }
}

pub struct Partition {
    /// The physical sector where the partition begins.
    pub start: u64,
    /// The size, in bytes, of a logical sector in the partition.
    pub sector_size: u64,
}

pub struct CachedDevice {
    device: Box<dyn BlockDevice>,
    cache: HashMap<u64, CacheEntry>,
    partition: Partition,
}

impl CachedDevice {
    /// Creates a new `CachedDevice` that transparently caches sectors from
    /// `device` and maps physical sectors to logical sectors inside of
    /// `partition`. All reads and writes from `CacheDevice` are performed on
    /// in-memory caches.
    ///
    /// The `partition` parameter determines the size of a logical sector and
    /// where logical sectors begin. An access to a sector `n` _before_
    /// `partition.start` is made to physical sector `n`. Cached sectors before
    /// `partition.start` are the size of a physical sector. An access to a
    /// sector `n` at or after `partition.start` is made to the _logical_ sector
    /// `n - partition.start`. Cached sectors at or after `partition.start` are
    /// the size of a logical sector, `partition.sector_size`.
    ///
    /// `partition.sector_size` must be an integer multiple of
    /// `device.sector_size()`.
    ///
    /// # Panics
    ///
    /// Panics if the partition's sector size is < the device's sector size.
    pub fn new<T>(device: T, partition: Partition) -> CachedDevice
    where
        T: BlockDevice + 'static,
    {
        assert!(partition.sector_size >= device.sector_size());

        CachedDevice {
            device: Box::new(device),
            cache: HashMap::new(),
            partition: partition,
        }
    }

    /// Maps a user's request for a sector `virt` to the physical sector and
    /// number of physical sectors required to access `virt`.
    fn virtual_to_physical(&self, virt: u64) -> (u64, u64) {
        if self.device.sector_size() == self.partition.sector_size {
            (virt, 1)
        } else if virt < self.partition.start {
            (virt, 1)
        } else {
            let factor = self.partition.sector_size / self.device.sector_size();
            let logical_offset = virt - self.partition.start;
            let physical_offset = logical_offset * factor;
            let physical_sector = self.partition.start + physical_offset;
            (physical_sector, factor)
        }
    }

    fn load_cache(&mut self, sector: u64) -> io::Result<()> {
        if !self.cache.contains_key(&sector) {
            let (physical_sector, factor) = self.virtual_to_physical(sector);
            let mut buf = Vec::with_capacity((self.device.sector_size() * factor) as usize);
            for i in 0..factor {
                self.device.read_all_sector(physical_sector + i, &mut buf)?;
            }
            self.cache.insert(sector, CacheEntry::new(buf));
        }
        Ok(())
    }

    /// Returns a mutable reference to the cached sector `sector`. If the sector
    /// is not already cached, the sector is first read from the disk.
    ///
    /// The sector is marked dirty as a result of calling this method as it is
    /// presumed that the sector will be written to. If this is not intended,
    /// use `get()` instead.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error reading the sector from the disk.
    pub fn get_mut(&mut self, sector: u64) -> io::Result<&mut [u8]> {
        self.load_cache(sector)?;
        let entry = self
            .cache
            .get_mut(&sector)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cache entry not found"))?;
        entry.dirty = true;
        Ok(entry.data.as_mut_slice())
    }

    /// Returns a reference to the cached sector `sector`. If the sector is not
    /// already cached, the sector is first read from the disk.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error reading the sector from the disk.
    pub fn get(&mut self, sector: u64) -> io::Result<&[u8]> {
        self.load_cache(sector)?;
        let entry = self
            .cache
            .get(&sector)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cache entry not found"))?;
        Ok(entry.data.as_slice())
    }
}

// FIXME: Implement `BlockDevice` for `CacheDevice`. The `read_sector` and
// `write_sector` methods should only read/write from/to cached sectors.
// TODO is it supposed to load the cache also or not???

impl BlockDevice for CachedDevice {
    fn sector_size(&self) -> u64 {
        self.partition.sector_size
    }
    fn read_sector(&mut self, n: u64, mut buf: &mut [u8]) -> io::Result<usize> {
        let data = self.get(n)?;
        buf.write(data)
        // let read = &self.cache[&n];
        // buf.write(&read.data[..])
    }
    fn write_sector(&mut self, n: u64, buf: &[u8]) -> io::Result<usize> {
        // let read = self.cache.get_mut(&n).unwrap();
        // read.data.write(&buf)
        let mut data = self.get_mut(n)?;
        data.write(buf)
    }
}

impl fmt::Debug for CachedDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CachedDevice")
            .field("device", &"<block device>")
            .field("cache", &self.cache)
            .finish()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn test_cache_read() {
        static mut DATA: [u8; 1024] = [0u8; 1024];
        // set some data
        unsafe {
            DATA[0] = 255;
            DATA[512] = 1;
        }
        let device = unsafe { Cursor::new(&mut DATA[..]) };
        let partition = Partition {
            start: 0,
            sector_size: 512,
        };
        let mut cache = CachedDevice::new(device, partition);
        let mut sector1 = [0u8; 512];
        assert_eq!(cache.read_sector(0, &mut sector1).unwrap(), 512);
        assert_eq!(sector1[0], 255);
        assert_eq!(cache.read_sector(1, &mut sector1).unwrap(), 512);
        assert_eq!(sector1[0], 1);
    }

    #[test]
    fn test_cache_write() {
        static mut DATA: [u8; 1024] = [0u8; 1024];
        // set some data
        unsafe {
            DATA[0] = 255;
        }
        let device = unsafe { Cursor::new(&mut DATA[..]) };
        let partition = Partition {
            start: 0,
            sector_size: 512,
        };
        let mut cache = CachedDevice::new(device, partition);
        let mut sector1 = [0u8; 512];
        sector1[0] = 1;
        assert_eq!(cache.write_sector(0, &sector1).unwrap(), 512);
        let mut read = [0u8; 512];
        assert_eq!(cache.read_sector(0, &mut read).unwrap(), 512);
        // should be 1 not 255
        assert_eq!(read[0], 1);
    }

    #[test]
    fn test_cache_bounds() {
        static mut DATA: [u8; 1024] = [0u8; 1024];
        let device = unsafe { Cursor::new(&mut DATA[..]) };
        let partition = Partition {
            start: 0,
            sector_size: 512,
        };
        let mut cache = CachedDevice::new(device, partition);
        let mut sector1 = [0u8; 512];
        cache
            .read_sector(3, &mut sector1)
            .expect_err("Out of bounds");
    }
}
