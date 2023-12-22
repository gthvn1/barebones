// https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Boot-information-format
use crate::drivers::uart::Serial;
use core::fmt::Write;

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

#[repr(C, packed)]
pub struct MemoryMapEntry {
    size: u32,
    addr_low: u32,
    addr_high: u32,
    len_low: u32,
    len_high: u32,
    typ: u32,
}

pub unsafe fn bootloader_name(info: *const BootInformation) {
    let mut com = Serial::new();
    let flags = (*info).flags;
    let name = (*info).boot_loader_name;

    if flags & 0b1000 != 0b1000 {
        // check bit 9
        writeln!(com, "bootloader name is not valid").unwrap();
        return;
    }

    let mut len = 0; // The name is null terminated according to multiboot spec
    while *name.add(len) != 0 {
        len += 1;
    }

    let s = core::str::from_utf8_unchecked(core::slice::from_raw_parts(name, len));
    writeln!(com, "bootloader name: {s}").unwrap();
}

pub unsafe fn print_mmap_sections(info: *const BootInformation) {
    let mut com = Serial::new();

    if (*info).flags & 0b100 != 0b100 {
        // check bit 6
        writeln!(com, "boot mmap entries are not valid").unwrap();
        return;
    }

    writeln!(com, "boot mmap entries").unwrap();

    let mmap_length = (*info).mmap_length;
    for i in 0..mmap_length {
        let p = ((*info).mmap_addr + core::mem::size_of::<MemoryMapEntry>() as u32 * i)
            as *const MemoryMapEntry;

        // The minimal size is 20 bytes. If the size is less than 20 bytes,
        // then the entry is invalid.
        if (*p).size < 20 {
            continue;
        }

        let len = (*p).len_low;
        let addr = (*p).addr_low;
        let typ = match (*p).typ {
            1 => "available RAM",
            3 => "ACPI reclaimable",
            4 => "reserved preserved on hibernation",
            5 => "bad memory",
            _ => "reserved",
        };

        writeln!(com, "-> len: {len:<10} | addr: {addr:#010x} | type:{typ}").unwrap();
    }
}
