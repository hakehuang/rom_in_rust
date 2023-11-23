#![no_main]
#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

use core::panic::PanicInfo;
use core::ptr;

// The reset handler
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    main();
}

#[no_mangle]
pub unsafe extern "C" fn FSL_ROM_OTP_INIT_ADDR_CALL(src_clk_freq: u32) -> i32 {
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn FSL_ROM_OTP_DEINIT_ADDR_CALL() -> i32 {
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn FSL_ROM_OTP_FUSE_READ_ADDR_CALL(addr: u32, data: *const u32) -> i32 {
    return 0;
}

fn main() -> ! {
    loop {
        // your code goes here
    }
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[link_section = ".OPT_API_CALL.FSL_ROM_OTP_INIT_ADDR"]
#[no_mangle]
pub static FSL_ROM_OTP_INIT_ADDR: unsafe extern "C" fn(src_clk_freq: u32) -> i32 =
    FSL_ROM_OTP_INIT_ADDR_CALL;

#[link_section = ".OPT_API_CALL.FSL_ROM_OTP_DEINIT_ADDR"]
#[no_mangle]
pub static FSL_ROM_OTP_DEINIT_ADDR: unsafe extern "C" fn() -> i32 = FSL_ROM_OTP_DEINIT_ADDR_CALL;

#[link_section = ".OPT_API_CALL.FSL_ROM_OTP_FUSE_READ_ADDR"]
#[no_mangle]
pub static FSL_ROM_OTP_FUSE_READ_ADDR: unsafe extern "C" fn(addr: u32, data: *const u32) -> i32 =
    FSL_ROM_OTP_FUSE_READ_ADDR_CALL;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
