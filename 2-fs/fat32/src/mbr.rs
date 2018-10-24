use std::{fmt, io, mem, ptr, slice};
use traits::BlockDevice;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CHS {
    head: u8,
    sector_upper: u8,
    cylinder_lower: u8,
}

impl CHS {
    pub fn sector(self) -> u8 {
        self.sector_upper & 0b0011_1111
    }
    pub fn cylinder(self) -> u16 {
        u16::from(self.sector_upper & 0b1100_0000) | u16::from(self.cylinder_lower)
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
#[derive(Clone)]
pub struct BootFlag(u8);

#[repr(C)]
#[derive(Clone, Copy)]
pub enum BootStatus {
    No,
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
}

impl BootFlag {
    pub fn status(&self) -> BootStatus {
        match self.0 {
            0x0 => BootStatus::No,
            0x80 => BootStatus::Active,
            _ => BootStatus::Unknown,
        }
    }
}

impl fmt::Debug for BootFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let active = match self.status() {
            BootStatus::Active => "ACTIVE",
            BootStatus::No => "INACTIVE",
            BootStatus::Unknown => "UNKNOWN",
        };
        write!(f, "BootFlag {}", active)
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
    pub fn is_fat32(&self) -> bool {
        match self.partition_type {
            0xB | 0xC => true,
            _ => false,
        }
    }
}

impl fmt::Debug for PartitionEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Parition Entry {}")
            .field("flag", &self.boot_flag)
            .field("CHS start", &self.chs_start)
            .field(
                "Partition type",
                &format_args!("{}", if self.is_fat32() { "FAT32" } else { "UNKNOWN" }),
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
    signature: u16,
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
    pub fn read_signature(&self) -> bool {
        match self.signature {
            MasterBootRecord::VALID_BOOTSECTOR => true,
            _ => false,
        }
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
        if !mbr.read_signature() {
            return Err(Error::BadSignature);
        }
        for (i, entry) in mbr.entries.iter().enumerate() {
            if !entry.boot_flag.status().is_unknown() {
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
                &format_args!("{}", unsafe {
                    std::str::from_utf8_unchecked(&self.disk_id)
                }),
            )
            .field("Partition Entries", &self.entries)
            .field(
                "Magic Signature",
                &format_args!(
                    "{}",
                    if self.read_signature() {
                        "VALID"
                    } else {
                        "INVALID "
                    },
                ),
            )
            .finish()
    }
}
