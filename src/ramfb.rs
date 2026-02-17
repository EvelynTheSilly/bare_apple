use core::ffi::CStr;
use core::mem;
use core::ptr::addr_of;

use crate::println;

#[repr(C)]
struct FWCfgFile {
    size: u32,
    select: u16,
    reserved: u16,
    name: [u8; 56],
}

#[repr(C, packed)]
struct FWCfgDmaAccess {
    control: u32,
    len: u32,
    addr: u64,
}

#[repr(C, packed)]
struct RamFBCfg {
    addr: u64,
    fmt: u32,
    flags: u32,
    w: u32,
    h: u32,
    st: u32,
}

const QEMU_CFG_DMA_CTL_READ: u32 = 0x02;
const QEMU_CFG_DMA_CTL_SELECT: u32 = 0x08;
const QEMU_CFG_DMA_CTL_WRITE: u32 = 0x10;

unsafe fn qemu_dma_transfer(control: u32, len: u32, addr: u64) {
    // Address of the DMA register on the aarch64 virt board
    let fw_cfg_dma: *mut u64 = 0x9020010 as *mut u64;
    let dma = FWCfgDmaAccess {
        control: control.to_be(),
        len: len.to_be(),
        addr: addr.to_be(),
    };
    unsafe {
        fw_cfg_dma.write_volatile((addr_of!(dma) as u64).to_be());
    }
    // Wait until DMA completed or error bit set
}

pub fn setup_ramfb(fb_addr: *mut u8, width: u32, height: u32) {
    let mut num_entries: u32 = 0;
    let fw_cfg_file_directory = 0x19;
    unsafe {
        qemu_dma_transfer(
            (fw_cfg_file_directory << 16 | QEMU_CFG_DMA_CTL_SELECT | QEMU_CFG_DMA_CTL_READ) as u32,
            mem::size_of::<u32>().try_into().unwrap(),
            addr_of!(num_entries) as u64,
        );
    }

    // QEMU DMA is BE so need to byte swap arguments and results on LE
    num_entries = num_entries.to_be();

    let ramfb = FWCfgFile {
        size: 0,
        select: 0,
        reserved: 0,
        name: [0; 56],
    };

    for _ in 0..num_entries {
        unsafe {
            qemu_dma_transfer(
                QEMU_CFG_DMA_CTL_READ,
                mem::size_of::<FWCfgFile>() as u32,
                addr_of!(ramfb) as u64,
            );
        }
        let entry = CStr::from_bytes_until_nul(&ramfb.name).unwrap();
        let entry = entry.to_str().unwrap();
        if entry == "etc/ramfb" {
            println!("found etc/ramfb");
            break;
        }
    }

    // See fourcc:
    // https://github.com/qemu/qemu/blob/54294b23e16dfaeb72e0ffa8b9f13ca8129edfce/include/standard-headers/drm/drm_fourcc.h#L188
    let pixel_format =
        ('R' as u32) | (('G' as u32) << 8) | (('2' as u32) << 16) | (('4' as u32) << 24);

    // Stride 0 means QEMU calculates from bpp_of_format*width:
    // https://github.com/qemu/qemu/blob/54294b23e16dfaeb72e0ffa8b9f13ca8129edfce/hw/display/ramfb.c#L60
    let ramfb_cfg = RamFBCfg {
        addr: (fb_addr as u64).to_be(),
        fmt: (pixel_format).to_be(),
        flags: (0 as u32).to_be(),
        w: (width as u32).to_be(),
        h: (height as u32).to_be(),
        st: (0 as u32).to_be(),
    };

    unsafe {
        qemu_dma_transfer(
            (ramfb.select.to_be() as u32) << 16 | QEMU_CFG_DMA_CTL_SELECT | QEMU_CFG_DMA_CTL_WRITE,
            mem::size_of::<RamFBCfg>() as u32,
            addr_of!(ramfb_cfg) as u64,
        );
    }
}
