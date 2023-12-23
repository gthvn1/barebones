// https://wiki.osdev.org/Serial_Ports
use core::{arch::asm, fmt, fmt::Write};
use lazy_static::lazy_static;
use spin::Mutex;

pub struct Serial {
    port: u16,
}

const COM1: u16 = 0x3F8;

// The problem is that the WRITER is immutable. So we use spinning mutex
// to make it mutable. We use lazy_static to initialize the WRITER.
lazy_static! {
    pub static ref SERIAL_WRITER: Mutex<Serial> = Mutex::new(Serial::new(COM1));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    SERIAL_WRITER.lock().write_fmt(args).unwrap();
}

#[allow(clippy::identity_op)]
#[allow(dead_code)]
impl Serial {
    pub fn new(port: u16) -> Serial {
        Serial::outb(port + 1, 0x00); // Disable all interrupts
        Serial::outb(port + 3, 0x80); // Enable DLAB (set baud rate divisor)
        Serial::outb(port + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
        Serial::outb(port + 1, 0x00); //                  (hi byte)
        Serial::outb(port + 3, 0x03); // 8 bits, no parity, one stop bit
        Serial::outb(port + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        Serial::outb(port + 4, 0x0B); // IRQs enabled, RTS/DSR set
        Serial::outb(port + 4, 0x1E); // Set in loopback mode, test the serial chip
        Serial::outb(port + 0, 0xAE); // Test serial chip (send byte 0xAE and check if
                                      // serial returns same byte)

        // Check if serial is faulty (i.e: not same byte as sent)
        if Serial::inb(port + 0) != 0xAE_u8 {
            panic!("Serial is faulty");
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        Serial::outb(port + 4, 0x0F);
        Serial { port }
    }

    pub fn write_string(&self, s: &str) {
        for ch in s.chars() {
            self.write_serial(ch as u8);
        }
    }

    // Receiving data.
    fn serial_received(&self) -> u8 {
        Serial::inb(self.port + 5) & 1_u8
    }

    fn read_serial(&self) -> u8 {
        while self.serial_received() == 0 {}

        Serial::inb(self.port)
    }

    // Sending data.
    fn is_transmit_empty(&self) -> u8 {
        Serial::inb(self.port + 5) & 0x20_u8
    }

    fn write_serial(&self, ch: u8) {
        while self.is_transmit_empty() == 0_u8 {}

        Serial::outb(self.port, ch);
    }

    // Input byte from port that is in DX to AL register.
    fn inb(port: u16) -> u8 {
        let ret: u8;
        unsafe {
            asm!(
                "inb %dx, %al",
                out("al") ret,
                in("dx") port,
                options(att_syntax)
            );
        }
        ret
    }

    // Output byte from AL register to port that is in DX.
    fn outb(port: u16, val: u8) {
        unsafe {
            asm!(
                "outb %al, %dx",
                in("dx") port,
                in("al") val,
                options(att_syntax)
            );
        }
    }
}

// Implementing Write trait for Serial.
// https://doc.rust-lang.org/core/macro.write.html
// The note says: Note: This macro can be used in no_std setups as well.
// In a no_std setup you are responsible for the implementation details
// of the components.
impl Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
