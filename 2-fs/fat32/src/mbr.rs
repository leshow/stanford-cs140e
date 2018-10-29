use std::{fmt, io, mem, str};
use traits::BlockDevice;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CHS {
    head: u8,
    sector_upper: u8,
    cylinder_lower: u8,
}

const UPPER_MASK: u8 = (-1i8 << 6) as u8;
const LOWER_MASK: u8 = (1 << 6) - 1 as u8;

impl CHS {
    pub fn sector(self) -> u8 {
        self.sector_upper & LOWER_MASK
    }
    pub fn cylinder(self) -> u16 {
        u16::from(self.sector_upper & UPPER_MASK) | u16::from(self.cylinder_lower)
    }
}

impl fmt::Debug for CHS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CHS")
            .field("Header", &self.head)
            .field("Sector", &self.sector())
            .field("Cylinder", &self.cylinder())
            .finish()
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum BootStatus {
    Inactive,
    Active,
    Unknown,
}

impl BootStatus {
    pub fn is_unknown(self) -> bool {
        match self {
            BootStatus::Unknown => true,
            _ => false,
        }
    }
    pub fn is_active(self) -> bool {
        match self {
            BootStatus::Active => true,
            _ => false,
        }
    }
    pub fn is_inactive(self) -> bool {
        match self {
            BootStatus::Inactive => true,
            _ => false,
        }
    }
}

impl fmt::Debug for BootStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let active = match *self {
            BootStatus::Active => "ACTIVE",
            BootStatus::Inactive => "INACTIVE",
            BootStatus::Unknown => "UNKNOWN",
        };
        write!(f, "BootFlag {}", active)
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct BootFlag(u8);

impl BootFlag {
    pub fn status(self) -> BootStatus {
        match self.0 {
            0x00 => BootStatus::Inactive,
            0x80 => BootStatus::Active,
            _ => BootStatus::Unknown,
        }
    }
    pub fn value(self) -> u8 {
        self.0
    }
}

impl fmt::Debug for BootFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BootFlag")
            .field("status", &format_args!("{:?}", self.status()))
            .finish()
    }
}

#[repr(C, packed)]
#[derive(Clone)]
pub struct PartitionEntry {
    boot_flag: BootFlag,
    chs_start: CHS,
    partition_type: u8,
    chs_end: CHS,
    sector_lba: u32,
    sector_total: u32,
}

impl PartitionEntry {
    pub fn is_fat(&self) -> bool {
        match self.partition_type {
            0xB | 0xC => true,
            _ => false,
        }
    }
    pub fn fat_begin_lba(&self) -> Option<u32> {
        if self.is_fat() {
            return Some(self.sector_lba + self.sector_total);
        }
        None
    }
}

impl fmt::Debug for PartitionEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Parition Entry {}")
            .field("flag", &self.boot_flag)
            .field("CHS start", &self.chs_start)
            .field(
                "Partition type",
                &format_args!("{}", if self.is_fat() { "FAT32" } else { "UNKNOWN" }),
            )
            .field("CHS end", &self.chs_end)
            .field("Sector LBA", &self.sector_lba)
            .field("Total sectors", &self.sector_total)
            .finish()
    }
}

/// The master boot record (MBR).
#[repr(C, packed)]
pub struct MasterBootRecord {
    bootstrap: [u8; 436],
    disk_id: [u8; 10],
    entries: [PartitionEntry; 4],
    boot_sig: u16,
}

#[derive(Debug)]
pub enum Error {
    /// There was an I/O error while reading the MBR.
    Io(io::Error),
    /// Partiion `.0` (0-indexed) contains an invalid or unknown boot indicator.
    UnknownBootIndicator(u8),
    /// The MBR magic signature was invalid.
    BadSignature,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl MasterBootRecord {
    pub const VALID_BOOTSECTOR: u16 = 0xAA55;

    pub fn has_mbr(&self) -> bool {
        self.boot_sig == MasterBootRecord::VALID_BOOTSECTOR
    }
    pub fn first_fat_partition(&self) -> Option<u32> {
        for entry in &self.entries {
            if let Some(lba_start) = entry.fat_begin_lba() {
                return Some(lba_start);
            }
        }
        None
    }
    pub fn fat_partition_start<'a>(&'a self) -> impl Iterator<Item = u32> + 'a {
        self.entries.iter().flat_map(|entry| entry.fat_begin_lba())
    }
    /// Reads and returns the master boot record (MBR) from `device`.
    ///
    /// # Errors
    ///
    /// Returns `BadSignature` if the MBR contains an invalid magic signature.
    /// Returns `UnknownBootIndicator(n)` if partition `n` contains an invalid
    /// boot indicator. Returns `Io(err)` if the I/O error `err` occured while
    /// reading the MBR.
    pub fn from<T: BlockDevice>(mut device: T) -> Result<MasterBootRecord, Error> {
        let mut buf = [0u8; 512];
        let size = device.read_sector(0, &mut buf)?;
        if size != 512 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "MBR did not read correct size",
            ))?;
        }
        let mbr = unsafe { mem::transmute::<_, MasterBootRecord>(buf) };
        if !mbr.has_mbr() {
            return Err(Error::BadSignature);
        }
        for (i, entry) in mbr.entries.iter().enumerate() {
            if entry.boot_flag.status().is_unknown() {
                return Err(Error::UnknownBootIndicator(i as u8));
            }
        }
        Ok(mbr)
    }
}
// reinterpreting structs:
// let ptr = buf.as_ptr() as *mut MasterBootRecord;
// simply read it as another value (moves value out of src)
// let _m = unsafe { ptr::read(ptr) };
// changes [u8;512] into &mut [MasterBootRecord], then we move MBR out
// let _m2 = unsafe {
//     let m = slice::from_raw_parts_mut(ptr, 512);
//     mem::replace(&mut m[0], mem::uninitialized())
// };
// same thing but with ptr::read
// let _m = unsafe {
//     let tmp = slice::from_raw_parts_mut(ptr, 512);
//     ptr::read(&tmp[0])
// };

impl fmt::Debug for MasterBootRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MBR")
            .field(
                "Disk ID",
                &format_args!("{}", unsafe { str::from_utf8_unchecked(&self.disk_id) }),
            )
            .field("Partition Entries", &self.entries)
            .field(
                "MBR: {}",
                &format_args!("{}", if self.has_mbr() { "YES" } else { "NO " },),
            )
            .finish()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn test_mbr_sig() {
        let mut buf = [0u8; 512];
        buf[510] = 0x55;
        buf[511] = 0xAA;
        let mbr = MasterBootRecord::from(Cursor::new(&mut buf[..])).unwrap();
        assert!(mbr.has_mbr());
    }
    #[test]
    fn test_mbr_invalid_sig() {
        let mut buf = [0u8; 512];
        buf[510] = 0x00;
        buf[511] = 0xAA;
        match MasterBootRecord::from(Cursor::new(&mut buf[..])) {
            Err(Error::BadSignature) => assert!(true),
            _ => assert!(false),
        }
    }
}
