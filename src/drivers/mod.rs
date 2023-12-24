#[macro_use]
pub mod uart;
pub mod vga;

use core::{fmt, fmt::Write};
use lazy_static::lazy_static;
use spin::Mutex;
use uart::{Serial, COM1};

// The problem is that the WRITER is immutable. So we use spinning mutex
// to make it mutable. We use lazy_static to initialize the WRITER.
lazy_static! {
    pub static ref WRITER: Mutex<Serial> = Mutex::new(Serial::new(COM1));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
