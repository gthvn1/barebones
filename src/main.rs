// By default the standard library is imported in all crates.
// Disable this.
#![no_std]
// When using standard library we have a main function that is the entry point.
// But without it we need to define our own entry point. So first we need to
// standard library to not use the default entry point.
#![no_main]
// and we will also need to tell to the compiler to not modify the name of our
// new entry point function using the 'mangle' attribute.

mod drivers;

use core::{arch::global_asm, panic::PanicInfo};
use drivers::serial;
use drivers::vga::TextMode;

global_asm!(include_str!("boot.s"), options(att_syntax));

// The panic_handler attribute is used to define a custom panic handler. As
// we don't have standard library we don't have the one provided by it that is
// the normal default handler. So we need to define our own
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // PanicInfo contains the file and line where the panic happened and
    // the optional panic message.
    loop {}
}

const BANNER: &str = "Welcome to Monkey Islang !\n\r";

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
pub extern "C" fn kernel_start() -> ! {
    let mut console = TextMode::new();

    console.clear();
    console.write_string(BANNER);
    console.write_string("How are you?\n\r");

    let com = serial::Serial::new();
    if com.init() {
        console.write_string("Serial port initialized\n\r");
        com.write_serial(b'H');
        com.write_serial(b'e');
        com.write_serial(b'l');
        com.write_serial(b'l');
        com.write_serial(b'o');
    } else {
        panic!("Serial port not initialized");
    };

    loop {}
}
