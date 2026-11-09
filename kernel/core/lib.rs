#![no_std]
#![no_main]

//! Minimal link-test entry point for B0.
//! This is not bootable until B1 adds the selected boot protocol and CPU setup.

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
