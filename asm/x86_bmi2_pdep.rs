extern crate bitintr;
use bitintr::*;

#[no_mangle]
pub fn pdep_u32(x: u32, mask: u32) -> u32 {
    x.pdep(mask)
}

#[no_mangle]
pub fn pdep_u64(x: u64, mask: u64) -> u64 {
    x.pdep(mask)
}
