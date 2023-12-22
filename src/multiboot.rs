#[repr(C)]
#[derive(Debug)]
pub struct BootInformation {
    flags: u32,
    mem_lower: u32,
    mem_upper: u32,
    boot_device: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    syms: [u32; 4],
    mmap_length: u32,
    mmap_addr: u32,
    drives_length: u32,
    drives_addr: u32,
    config_table: u32,
    pub boot_loader_name: *const u8,
    // We don't need to know the rest of the struct
}
impl BootInformation {
    pub unsafe fn bootloader_name(info: &BootInformation) -> &str {
        let ptr = info.boot_loader_name;
        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }
        core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len))
    }
}
