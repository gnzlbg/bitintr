extern crate bitintr;
use bitintr::*;

#[no_mangle]
pub fn rbit_u32(x: u32) -> u32 {
    x.rbit()
}
