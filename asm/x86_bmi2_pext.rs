extern crate bitintr;
use bitintr::*;

#[no_mangle]
pub fn pext_u32(x: u32, mask: u32) -> u32 {
    x.pext(mask)
}

#[no_mangle]
pub fn pext_u64(x: u64, mask: u64) -> u64 {
    x.pext(mask)
}
