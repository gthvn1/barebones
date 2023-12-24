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
use memory::multiboot::{print_bootloader_name, print_mmap_sections, BootInformation};

global_asm!(include_str!("boot.s"), options(att_syntax));

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
    println!("eax: {:#010x}", eax);
    println!("ebx: {:#010x}", ebx as u32);
    unsafe {
        print_bootloader_name(ebx);
        print_mmap_sections(ebx);
    }

    #[cfg(test)]
    test_main();

    exit_qemu(QemuExitCode::Success);
    unreachable!();
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
