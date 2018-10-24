use std::{fmt, io};

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
        self.sector_upper & 0x0011_1111
    }
    pub fn cylinder(self) -> u16 {
        (self.sector_upper & 0x1100_0000) as u16 | self.cylinder_lower as u16
    }
}

impl fmt::Debug for CHS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CHS")
            .field("header: {}", &self.head)
            .field("sector: {}", &self.sector())
            .field("cylinder: {}", &self.cylinder())
            .finish()
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct BootFlag(u8);

impl BootFlag {
    pub fn active(&self) -> bool {
        match self.0 {
            0x0 => false,
            0x80 => true,
            _ => false,
        }
    }
}

impl fmt::Debug for BootFlag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let active = if self.active() { "ACTIVE" } else { "INACTIVE" };
        write!(f, "BootFlag: {}", active)
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
        let active = if self.is_fat32() { "FAT32" } else { "UNKNOWN" };
        write!(f, "Partition Type: {}", active);
        f.debug_struct("Parition Entry: {}")
            .field("flag: {:?}", &self.boot_flag)
            .field("CHS start: {:?}", &self.chs_start)
            .field("CHS end: {:?}", &self.chs_end)
            .field("Sector LBA: {}", &self.sector_lba)
            .field("Total sectors: {}", &self.sector_total)
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

impl MasterBootRecord {
    pub const VALID_BOOTSECTOR: u16 = 0xAA55;
    /// Reads and returns the master boot record (MBR) from `device`.
    ///
    /// # Errors
    ///
    /// Returns `BadSignature` if the MBR contains an invalid magic signature.
    /// Returns `UnknownBootIndicator(n)` if partition `n` contains an invalid
    /// boot indicator. Returns `Io(err)` if the I/O error `err` occured while
    /// reading the MBR.
    pub fn from<T: BlockDevice>(mut device: T) -> Result<MasterBootRecord, Error> {
        unimplemented!("MasterBootRecord::from()")
    }
}

impl fmt::Debug for MasterBootRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!("MasterBootRecord::fmt()")
    }
}
