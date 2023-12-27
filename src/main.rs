// By default the standard library is imported in all crates.
// Disable this.
#![no_std]
// When using standard library we have a main function that is the entry point.
// But without it we need to define our own entry point. So first we need to
// standard library to not use the default entry point.
#![no_main]
// and we will also need to tell to the compiler to not modify the name of our
// new entry point function using the 'mangle' attribute.
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// function name to use for the test runner is ignored because we are using the
// no_main attribute. So we need to tell the compiler to use the test_main
#![reexport_test_harness_main = "test_main"]

#[macro_use]
mod drivers;
mod memory;

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};
use drivers::vga::TextMode;
use memory::multiboot::{get_mem_from_multiboot, print_bootloader_name, BootInformation};

global_asm!(include_str!("boot.s"), options(att_syntax));

extern "C" {
    static _stext: u32;
    static _etext: u32;
    static _srodata: u32;
    static _erodata: u32;
    static _sdata: u32;
    static _edata: u32;
    static _sbss: u32;
    static _ebss: u32;
}

// The panic_handler attribute is used to define a custom panic handler. As
// we don't have standard library we don't have the one provided by it that is
// the normal default handler. So we need to define our own
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // PanicInfo contains the file and line where the panic happened and
    // the optional panic message.
    println!("{}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {} // This is needed because the compiler doesn't know that exit_qemu
            // never returns.
}

// Quitting qemu properly
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x0,
    Failed = 0x1,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    const PORT: u16 = 0xf4;
    unsafe {
        asm!(
            "outb %al, %dx",
            in("dx") PORT,
            in("al") exit_code as u8,
            options(att_syntax)
        );
    }
}

const BANNER: &str = "\n\r -=( Welcome to Monkey Islang ! )=-\n\r";

// Providing our panic handler generates an error about 'eh_personality'.
// The eh stands for exception handling. 'eh_personality' marks
// a function that is used for implementing stack unwinding. It is a complex
// process that requires some OS support. So we don't want to use it for now.
// We disabled it in Cargo.toml by setting the 'panic=abort' option.

// The _start function is the entry point of our program. We mark it as extern
// "C" to tell the compiler to use the C calling convention for this function.
// _start is the name of the entry point for most systems.
// The '!' type means that this function never returns.
#[allow(clippy::empty_loop)]
#[no_mangle]
pub extern "C" fn kernel_start(eax: u32, ebx: *const BootInformation) -> ! {
    let mut console = TextMode::new();

    console.clear();
    console.write_string(BANNER);
    console.write_string("\r\n Ouputs are redirected to uart (COM1)");
    console.write_string("\r\n If you don't see them check that ");
    console.write_string("serial is enabled in qemu...");

    println!("{}", BANNER);

    println!("# SETUP MEMORY");

    println!("## Registers");
    println!("eax: {:#010x}", eax);
    println!("ebx: {:#010x}", ebx as u32);
    unsafe {
        let esp: u32;
        asm!("mov {}, esp", out(reg) esp);
        println!("esp: {:#010x}", esp);
    }

    println!("## Memory areas");
    let text_start = unsafe { &_stext as *const u32 as usize };
    let text_end = unsafe { &_etext as *const u32 as usize };
    let ro_data_start = unsafe { &_srodata as *const u32 as usize };
    let ro_data_end = unsafe { &_erodata as *const u32 as usize };
    let data_start = unsafe { &_sdata as *const u32 as usize };
    let data_end = unsafe { &_edata as *const u32 as usize };
    let bss_start = unsafe { &_sbss as *const u32 as usize };
    let bss_end = unsafe { &_ebss as *const u32 as usize };

    println!(
        "text_area   : start {:#010x} -> end {:#010x} : {}",
        text_start,
        text_end,
        text_end - text_start
    );
    println!(
        "ro_data_area: start {:#010x} -> end {:#010x} : {}",
        ro_data_start,
        ro_data_end,
        ro_data_end - ro_data_start
    );
    println!(
        "data_area   : start {:#010x} -> end {:#010x} : {}",
        data_start,
        data_end,
        data_end - data_start
    );
    println!(
        "bss_area    : start {:#010x} -> end {:#010x} : {}",
        bss_start,
        bss_end,
        bss_end - bss_start
    );

    let mut mem_start = 0;
    let mut mem_len = 0;

    println!("## Multiboot");
    unsafe {
        print_bootloader_name(ebx);
        (mem_start, mem_len) = get_mem_from_multiboot(ebx);
    }

    println!("## Setup Memory");
    let mem_len_mo = mem_len >> 20;
    println!("Reclaiming {mem_len_mo}Mo from {mem_start:#010x}");

    // TODO: init memory

    println!("# ALL DONE");

    #[cfg(test)]
    test_main();

    exit_qemu(QemuExitCode::Success);
    unreachable!();
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
}
