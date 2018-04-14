extern crate bitintr;
use bitintr::*;

#[no_mangle]
pub fn cls_u32(x: u32) -> u32 {
    x.cls()
}

#[no_mangle]
pub fn cls_u64(x: u64) -> u64 {
    x.cls()
}
