extern crate bitintr;

#[no_mangle]
pub fn rbit_u64(x: u64) -> u64 {
    bitintr::arm::v7::rbit(x)
}
