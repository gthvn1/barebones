pub fn hello() {
    let vga_buffer = 0xb8000 as *mut u8;
    const HELLO: &[u8] = b"Welcome to Monkey Islang!";

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte as u8;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb as u8;
        }
    }
}
