use std::fmt;
use vfat::*;

use self::Status::*;

#[derive(Debug, PartialEq)]
pub enum Status {
    /// The FAT entry corresponds to an unused (free) cluster.
    Free,
    /// The FAT entry/cluster is reserved.
    Reserved,
    /// The FAT entry corresponds to a valid data cluster. The next cluster in
    /// the chain is `Cluster`.
    Data(Cluster),
    /// The FAT entry corresponds to a bad (disk failed) cluster.
    Bad,
    /// The FAT entry corresponds to a valid data cluster. The corresponding
    /// cluster is the last in its chain.
    Eoc(u32),
}

#[repr(C, packed)]
pub struct FatEntry(pub u32);

impl FatEntry {
    /// Returns the `Status` of the FAT entry `self`.
    pub fn status(&self) -> Status {
        match self.0 & 0x0FFF_FFFF {
            0x0000_0000 => Free,
            0x0000_0001 => Reserved,
            clus @ 0x0000_0002...0x0FFF_FFEF => Data(Cluster::from(clus)),
            clus @ 0x0FFF_FFF0...0x0FFF_FFF5 => Data(Cluster::from(clus)),
            0x0FFF_FFF6 => Reserved,
            0x0FFF_FFF7 => Bad,
            marker @ 0x0FFF_FFF8...0xFFFF_FFFF => Eoc(marker),
            _ => unreachable!(),
        }
    }
}

impl fmt::Debug for FatEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FatEntry")
            .field("value", &self.0)
            .field("status", &self.status())
            .finish()
    }
}
