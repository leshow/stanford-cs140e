use std::{fmt, io, mem};

use traits::BlockDevice;
use vfat::Error;

#[repr(C, packed)]
pub struct BiosParameterBlock {
    _r1: [u8; 3],
    oem_ident: [u8; 8],
    bytes_per_sec: u16,
    sec_per_clus: u8,
    rsvd_sec_count: u16,
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
    pub const VALID_SIG: u16 = 0xAA55;

    pub fn valid(&self) -> bool {
        self.boot_sig == BiosParameterBlock::VALID_SIG
    }
    pub fn bytes_per_sec(&self) -> u16 {
        self.bytes_per_sec
    }
    pub fn sec_per_clus(&self) -> u8 {
        self.sec_per_clus
    }
    pub fn fat_start(&self) -> u64 {
        u64::from(self.rsvd_sec_count)
    }
    pub fn num_fats(&self) -> u8 {
        self.num_fats
    }
    pub fn sec_per_fat(&self) -> u32 {
        self.sec_per_fat_size
    }
    pub fn root_clus(&self) -> u32 {
        self.clus_num_root
    }
    pub fn data_start_sec(&self) -> u64 {
        // begin lba
        self.fat_start() + (u64::from(self.num_fats()) * u64::from(self.sec_per_fat()))
    }
    pub fn logical_sec(&self) -> u32 {
        if self.num_logical_sec_bpb == 0 {
            self.num_logical_sec_epbp
        } else {
            u32::from(self.num_logical_sec_bpb)
        }
    }

    /// Reads the FAT32 extended BIOS parameter block from sector `sector` of
    /// device `device`.
    ///
    /// # Errors
    ///
    /// If the EBPB signature is invalid, returns an error of `BadSignature`.
    pub fn from<T: BlockDevice>(mut device: T, sector: u64) -> Result<BiosParameterBlock, Error> {
        let mut buf = [0u8; 512];
        let size = device.read_sector(sector, &mut buf)?;
        if size != 512 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "BPB did not get correct size",
            ))?;
        }
        let bpb = unsafe { mem::transmute::<_, BiosParameterBlock>(buf) };
        if !bpb.valid() {
            return Err(Error::BadSignature);
        }
        Ok(bpb)
    }
}

impl fmt::Debug for BiosParameterBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BPB")
            .field("Bytes per sector", &self.bytes_per_sec())
            .field("Sectors per cluster", &self.sec_per_clus())
            .field("FAT start", &self.fat_start())
            .field("Num FATs", &self.num_fats())
            .field("Sectors per FAT", &self.sec_per_fat())
            .field("Root cluster", &self.root_clus())
            .field("Data Start (lba start)", &self.data_start_sec())
            .field("Logical Start", &self.logical_sec())
            .field(
                "Signature",
                &format_args!("{}", if self.valid() { "VALID" } else { "INVALID" }),
            )
            .field(
                "Volume label",
                &format_args!("{}", unsafe {
                    std::str::from_utf8_unchecked(&self.vol_label)
                }),
            )
            .finish()
    }
}
