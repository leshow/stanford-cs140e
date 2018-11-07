use std::cmp::min;
use std::io;
use std::mem::size_of;
use std::path::Path;

use mbr::MasterBootRecord;
use traits::{BlockDevice, FileSystem};
use util::SliceExt;
use vfat::{BiosParameterBlock, CachedDevice, Partition};
use vfat::{Cluster, Dir, Entry, Error, FatEntry, File, Shared, Status};

#[derive(Debug)]
pub struct VFat {
    device: CachedDevice,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    sectors_per_fat: u32,
    fat_start_sector: u64,
    data_start_sector: u64,
    root_dir_cluster: Cluster,
}

impl VFat {
    pub fn from<T>(mut device: T) -> Result<Shared<VFat>, Error>
    where
        T: BlockDevice + 'static,
    {
        let mbr = MasterBootRecord::from(&mut device)?;
        let vfat_start = mbr.first_fat_partition();
        if vfat_start.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No partition start found",
            ))?;
        }
        let vfat_start = u64::from(vfat_start.unwrap());
        let ebpb = BiosParameterBlock::from(&mut device, vfat_start)?;
        let device = CachedDevice::new(
            device,
            Partition {
                sector_size: ebpb.bytes_per_sec().into(),
                start: vfat_start,
            },
        );
        let root_dir_cluster = Cluster::from(ebpb.root_clus());

        Ok(Shared::new(VFat {
            device,
            bytes_per_sector: ebpb.bytes_per_sec(),
            sectors_per_cluster: ebpb.sec_per_clus(),
            sectors_per_fat: ebpb.sec_per_fat(),
            fat_start_sector: vfat_start + ebpb.fat_start(),
            data_start_sector: vfat_start + ebpb.data_start_sec(),
            root_dir_cluster,
        }))
    }

    // TODO: The following methods may be useful here:
    //
    //  * A method to read from an offset of a cluster into a buffer.
    //
    fn read_cluster(
        &mut self,
        cluster: Cluster,
        offset: usize,
        buf: &mut [u8],
    ) -> io::Result<usize> {
        unimplemented!()
    }

    //  * A method to read all of the clusters chained from a starting cluster
    //    into a vector.
    //
    fn read_chain(&mut self, start: Cluster, buf: &mut Vec<u8>) -> io::Result<usize> {
        unimplemented!()
    }

    //  * A method to return a reference to a `FatEntry` for a cluster where the
    //    reference points directly into a cached sector.
    //
    fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry> {
        unimplemented!()
    }
}

impl<'a> FileSystem for &'a Shared<VFat> {
    type File = ::traits::Dummy;
    type Dir = ::traits::Dummy;
    type Entry = ::traits::Dummy;

    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<Self::Entry> {
        unimplemented!("FileSystem::open()")
    }

    fn create_file<P: AsRef<Path>>(self, _path: P) -> io::Result<Self::File> {
        unimplemented!("read only file system")
    }

    fn create_dir<P>(self, _path: P, _parents: bool) -> io::Result<Self::Dir>
    where
        P: AsRef<Path>,
    {
        unimplemented!("read only file system")
    }

    fn rename<P, Q>(self, _from: P, _to: Q) -> io::Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        unimplemented!("read only file system")
    }

    fn remove<P: AsRef<Path>>(self, _path: P, _children: bool) -> io::Result<()> {
        unimplemented!("read only file system")
    }
}
