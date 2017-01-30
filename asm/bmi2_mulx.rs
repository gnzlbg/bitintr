extern crate bitintr;

#[no_mangle]
pub fn umulx_u32(x: u32, y: u32) -> (u32, u32) {
    bitintr::x86::bmi2::mulx(x, y)
}

#[no_mangle]
pub fn umulx_u64(x: u64, y: u64) -> (u64, u64) {
    bitintr::x86::bmi2::mulx(x, y)
}
