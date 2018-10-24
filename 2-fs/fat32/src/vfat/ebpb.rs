use std::fmt;

use traits::BlockDevice;
use vfat::Error;

#[repr(C, packed)]
pub struct BiosParameterBlock {
    _r1: [u8; 3],
    oem_ident: [u8; 8],
    bytes_per_sec: u16,
    sec_per_clus: u8,
    sec_reserved: u16,
    num_fats: u8,
    num_dir_entries: u16,
    num_logical_sec_bpb: u16,
    fat_id: u8,
    num_sec_per_fat: u16,
    sec_per_track: u16,
    heads_num: u16,
    num_hidden_sec: u32,
    num_logical_sec_epbp: u32,
    // epbp
    sec_per_fat_size: u32,
    flags: u16,
    vfat_version: u16,
    clus_num_root: u32,
    sec_num_fsinfo: u16,
    sec_num_backup_boot: u16,
    _r2: [u8; 12],
    drive_num: u8,
    nt_flags: u8,
    signature: u8,
    vol_id: u32,
    vol_label: [u8; 11],
    sys_ident: [u8; 8],
    boot_code: [u8; 420],
    boot_sig: u16,
}

impl BiosParameterBlock {
    /// Reads the FAT32 extended BIOS parameter block from sector `sector` of
    /// device `device`.
    ///
    /// # Errors
    ///
    /// If the EBPB signature is invalid, returns an error of `BadSignature`.
    pub fn from<T: BlockDevice>(mut device: T, sector: u64) -> Result<BiosParameterBlock, Error> {
        unimplemented!("BiosParameterBlock::from()")
    }
}

impl fmt::Debug for BiosParameterBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!("BiosParameterBlock::debug()")
    }
}
