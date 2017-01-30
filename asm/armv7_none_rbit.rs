extern crate bitintr;

#[no_mangle]
pub fn rbit_u32(x: u32) -> u32 {
    bitintr::arm::v7::rbit(x)
}
