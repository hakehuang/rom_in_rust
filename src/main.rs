#![no_main]
#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate cortex_m_rt as rt;
extern crate panic_halt;

use rt::entry;

const FFRO_STABLE_TIME: u32 = 12;
const SFRO_STABLE_TIME: u32 = 13;
const FIRC_48MHZ_TRIM_TEMPCO: u32 = 48;
const FIRC_48MHZ_TRIM_COARSE: u32 = 49;
const FIRC_48MHZ_TRIM_FINE: u32 = 50;
const FIRC_60MHZ_TRIM_TEMPCO: u32 = 51;
const FIRC_60MHZ_TRIM_COARSE: u32 = 52;
const FIRC_60MHZ_TRIM_FINE: u32 = 53;

#[no_mangle]
#[link_section = ".ROM_API_0.FSL_ROM_OTP_INIT_ADDR"]
pub unsafe extern "C" fn FSL_ROM_OTP_INIT_ADDR(src_clk_freq: u32) -> i32 {
    return 0;
}

#[no_mangle]
#[link_section = ".ROM_API_1.FSL_ROM_OTP_DEINIT_ADDR"]
pub unsafe extern "C" fn FSL_ROM_OTP_DEINIT_ADDR() -> i32 {
    return 0;
}
#[no_mangle]
#[link_section = ".ROM_API_2.FSL_ROM_OTP_FUSE_READ_ADDR"]
pub unsafe extern "C" fn FSL_ROM_OTP_FUSE_READ_ADDR(addr: u32, data: &mut u32) -> i32 {
    /*all fake data here*/
    match addr {
        FFRO_STABLE_TIME => *data = 0x10,
        SFRO_STABLE_TIME => *data = 0x10,
        FIRC_48MHZ_TRIM_TEMPCO => *data = 0x10,
        FIRC_48MHZ_TRIM_COARSE => *data = 0x20,
        FIRC_48MHZ_TRIM_FINE => *data = 0x40,
        FIRC_60MHZ_TRIM_TEMPCO => *data = 0x10,
        FIRC_60MHZ_TRIM_COARSE => *data = 0x20,
        FIRC_60MHZ_TRIM_FINE => *data = 0x40,
        _ => *data = 0,
    }

    return 0;
}

#[entry]
fn main() -> ! {
    loop {
        // your code goes here
    }
}
