extern crate bitintr;

#[no_mangle]
pub fn rbit_u8(x: u8) -> u8 {
    bitintr::arm::v7::rbit(x)
}

#[no_mangle]
pub fn rbit_u32(x: u32) -> u32 {
    bitintr::arm::v7::rbit(x)
}

#[no_mangle]
pub fn rbit_u64(x: u64) -> u64 {
    bitintr::arm::v7::rbit(x)
}
