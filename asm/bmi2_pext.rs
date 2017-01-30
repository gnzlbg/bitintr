extern crate bitintr;

#[no_mangle]
pub fn pext_u32(x: u32, mask: u32) -> u32 {
    bitintr::x86::bmi2::pext(x, mask)
}

#[no_mangle]
pub fn pext_u64(x: u64, mask: u64) -> u64 {
    bitintr::x86::bmi2::pext(x, mask)
}
