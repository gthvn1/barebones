// https://wiki.osdev.org/Serial_Ports
use core::arch::asm;

pub struct Serial {
    port: u16,
}

#[allow(clippy::identity_op)]
#[allow(dead_code)]
impl Serial {
    pub fn new() -> Serial {
        Serial { port: 0x3f8 }
    }

    pub fn init(&self) -> bool {
        Serial::outb(self.port + 1, 0x00); // Disable all interrupts
        Serial::outb(self.port + 3, 0x80); // Enable DLAB (set baud rate divisor)
        Serial::outb(self.port + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
        Serial::outb(self.port + 1, 0x00); //                  (hi byte)
        Serial::outb(self.port + 3, 0x03); // 8 bits, no parity, one stop bit
        Serial::outb(self.port + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        Serial::outb(self.port + 4, 0x0B); // IRQs enabled, RTS/DSR set
        Serial::outb(self.port + 4, 0x1E); // Set in loopback mode, test the serial chip
        Serial::outb(self.port + 0, 0xAE); // Test serial chip (send byte 0xAE and check if
                                           // serial returns same byte)

        // Check if serial is faulty (i.e: not same byte as sent)
        if Serial::inb(self.port + 0) != 0xAE_u8 {
            return false;
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        Serial::outb(self.port + 4, 0x0F);
        true
    }

    // Receiving data.
    pub fn serial_received(&self) -> u8 {
        Serial::inb(self.port + 5) & 1_u8
    }

    pub fn read_serial(&self) -> u8 {
        while self.serial_received() == 0 {}

        Serial::inb(self.port)
    }

    // Sending data.
    pub fn is_transmit_empty(&self) -> u8 {
        Serial::inb(self.port + 5) & 0x20_u8
    }

    pub fn write_serial(&self, ch: u8) {
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
