// https://wiki.osdev.org/Interrupts_Tutorial

// 32 bit IDT entry
// If we switch to 64 bits we need to change this
#[allow(dead_code)]
#[repr(C, packed)]
pub struct IdtEntry {
    isr_low: u16,   // lower 16 bits of ISR
    kernel_cs: u16, // GDT selector
    reserved: u8,
    attributes: u8,
    isr_high: u16, // higher 16 bits of ISR
}

// Note: Since we don’t know when an exception occurs, we can’t backup any
// registers before. This means we can’t use a calling convention that relies
// on caller-saved registers for exception handlers. Instead, we need a calling
// convention that preserves all registers. The x86-interrupt calling convention
// is such a calling convention, so it guarantees that all register values are
// restored to their original values on function return.
