extern crate bitintr;
use bitintr::*;

#[no_mangle]
pub fn umulx_u32(x: u32, y: u32) -> (u32, u32) {
    x.mulx(y)
}

#[no_mangle]
pub fn umulx_u64(x: u64, y: u64) -> (u64, u64) {
    x.mulx(y)
}
