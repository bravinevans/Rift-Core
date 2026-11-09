#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::ptr::{addr_of, read_volatile};

global_asm!(
    include_str!("../arch/x86_64/entry.s"),
    options(att_syntax)
);

const COM1_PORT: u16 = 0x3F8;

const LIMINE_REQUESTS_START_MARKER: [u64; 4] = [
    0xf6b8_f4b3_9de7_d1ae,
    0xfab9_1a69_40fc_b9cf,
    0x785c_6ed0_15d3_e316,
    0x181e_920a_7852_b9d9,
];
const LIMINE_REQUESTS_END_MARKER: [u64; 2] = [
    0xadc0_e053_1bb1_0d03,
    0x9572_709f_3176_4c62,
];

const LIMINE_COMMON_MAGIC_0: u64 = 0xc7b1_dd30_df4c_8b88;
const LIMINE_COMMON_MAGIC_1: u64 = 0x0a82_e883_a194_f07b;

const LIMINE_FRAMEBUFFER_REQUEST_ID_2: u64 = 0x9d58_27dc_d881_dd75;
const LIMINE_FRAMEBUFFER_REQUEST_ID_3: u64 = 0xa314_8604_f6fa_b11b;
const LIMINE_MEMMAP_REQUEST_ID_2: u64 = 0x67cf_3d9d_378a_806f;
const LIMINE_MEMMAP_REQUEST_ID_3: u64 = 0xe304_acdf_c50c_3c62;

#[repr(C)]
struct LimineFramebuffer {
    address: *mut u8,
    width: u64,
    height: u64,
    pitch: u64,
    bpp: u16,
    memory_model: u8,
    red_mask_size: u8,
    red_mask_shift: u8,
    green_mask_size: u8,
    green_mask_shift: u8,
    blue_mask_size: u8,
    blue_mask_shift: u8,
    unused: [u8; 7],
    edid_size: u64,
    edid: *mut u8,
    mode_count: u64,
    modes: *mut *mut u8,
}

#[repr(C)]
struct LimineFramebufferResponse {
    revision: u64,
    framebuffer_count: u64,
    framebuffers: *mut *mut LimineFramebuffer,
}

#[repr(C)]
struct LimineFramebufferRequest {
    id: [u64; 4],
    revision: u64,
    response: *mut LimineFramebufferResponse,
}

#[repr(C)]
struct LimineMemmapEntry {
    base: u64,
    length: u64,
    kind: u64,
}

#[repr(C)]
struct LimineMemmapResponse {
    revision: u64,
    entry_count: u64,
    entries: *mut *mut LimineMemmapEntry,
}

#[repr(C)]
struct LimineMemmapRequest {
    id: [u64; 4],
    revision: u64,
    response: *mut LimineMemmapResponse,
}

/* Limine changes the last base-revision value to zero when it supports it. */
#[used]
#[link_section = ".limine_requests"]
static mut BASE_REVISION: [u64; 3] = [
    0xf956_2b2d_5c95_a6c8,
    0x6a7b_3849_4453_6bdc,
    6,
];

#[used]
#[link_section = ".limine_requests_start"]
static REQUESTS_START: [u64; 4] = LIMINE_REQUESTS_START_MARKER;

#[used]
#[link_section = ".limine_requests"]
static mut FRAMEBUFFER_REQUEST: LimineFramebufferRequest = LimineFramebufferRequest {
    id: [
        LIMINE_COMMON_MAGIC_0,
        LIMINE_COMMON_MAGIC_1,
        LIMINE_FRAMEBUFFER_REQUEST_ID_2,
        LIMINE_FRAMEBUFFER_REQUEST_ID_3,
    ],
    revision: 0,
    response: core::ptr::null_mut(),
};

#[used]
#[link_section = ".limine_requests"]
static mut MEMMAP_REQUEST: LimineMemmapRequest = LimineMemmapRequest {
    id: [
        LIMINE_COMMON_MAGIC_0,
        LIMINE_COMMON_MAGIC_1,
        LIMINE_MEMMAP_REQUEST_ID_2,
        LIMINE_MEMMAP_REQUEST_ID_3,
    ],
    revision: 0,
    response: core::ptr::null_mut(),
};

#[used]
#[link_section = ".limine_requests_end"]
static REQUESTS_END: [u64; 2] = LIMINE_REQUESTS_END_MARKER;

#[no_mangle]
pub extern "C" fn rust_kernel_main() -> ! {
    if !limine_responses_are_valid() {
        serial_write_str("Rift Core: Limine setup failed.\n");
        halt_forever();
    }

    serial_write_str("Rift Core: booting with Limine.\n");
    serial_write_str("Rift Core: memory map response received.\n");
    kmain()
}

fn limine_responses_are_valid() -> bool {
    unsafe {
        let base_revision_ptr = addr_of!(BASE_REVISION).cast::<u64>().add(2);
        if read_volatile(base_revision_ptr) != 0 {
            return false;
        }

        let memmap_response = read_volatile(addr_of!(MEMMAP_REQUEST.response));
        if memmap_response.is_null() {
            return false;
        }

        let entry_count = read_volatile(addr_of!((*memmap_response).entry_count));
        let entries = read_volatile(addr_of!((*memmap_response).entries));
        if entry_count != 0 && entries.is_null() {
            return false;
        }

        /* A framebuffer is optional; if supplied, validate its pointer array. */
        let framebuffer_response = read_volatile(addr_of!(FRAMEBUFFER_REQUEST.response));
        if !framebuffer_response.is_null() {
            let framebuffer_count =
                read_volatile(addr_of!((*framebuffer_response).framebuffer_count));
            let framebuffers = read_volatile(addr_of!((*framebuffer_response).framebuffers));
            if framebuffer_count != 0 && framebuffers.is_null() {
                return false;
            }
        }
    }

    true
}

fn kmain() -> ! {
    serial_write_str("Rift Core: kernel entry reached.\n");
    halt_forever()
}

fn serial_write_str(message: &str) {
    for byte in message.bytes() {
        serial_write_byte(byte);
    }
}

fn serial_write_byte(byte: u8) {
    unsafe {
        while read_volatile((COM1_PORT + 5) as *const u8) & 0x20 == 0 {
            core::hint::spin_loop();
        }
        core::ptr::write_volatile(COM1_PORT as *mut u8, byte);
    }
}

fn halt_forever() -> ! {
    loop {
        unsafe {
            asm!("cli", "hlt", options(nomem, nostack));
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    serial_write_str("Rift Core panic.\n");
    halt_forever()
}
