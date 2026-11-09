#![no_std]
#![no_main]

use core::{arch::asm, ptr};

const COM1_PORT: u16 = 0x3F8;

extern "C" {
    static __bss_start: u8;
    static __bss_end: u8;
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let bss_start = &__bss_start as *const u8 as *mut u8;
        let bss_end = &__bss_end as *const u8;
        let len = bss_end.offset_from(bss_start) as usize;
        ptr::write_bytes(bss_start, 0, len);
    }

    serial_write_str("Rift Core: booting...\n");
    kmain();
}

fn kmain() -> ! {
    serial_write_str("Rift Core: kernel entry reached.\n");
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

fn serial_write_str(message: &str) {
    for byte in message.bytes() {
        serial_write_byte(byte);
    }
}

fn serial_write_byte(byte: u8) {
    unsafe {
        while ptr::read_volatile((COM1_PORT + 5) as *const u8) & 0x20 == 0 {
            core::hint::spin_loop();
        }
        ptr::write_volatile(COM1_PORT as *mut u8, byte);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    serial_write_str("Rift Core panic: ");
    if let Some(msg) = info.message().as_str() {
        serial_write_str(msg);
    }
    serial_write_str("\n");

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
