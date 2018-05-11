extern crate bitintr;
use bitintr::*;

#[no_mangle]
pub fn rbit_u64(x: u64) -> u64 {
    x.rbit()
}
