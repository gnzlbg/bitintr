extern crate bitintr;

#[no_mangle]
pub fn pdep_u32(x: u32, mask: u32) -> u32 {
    bitintr::x86::bmi2::pdep(x, mask)
}

#[no_mangle]
pub fn pdep_u64(x: u64, mask: u64) -> u64 {
    bitintr::x86::bmi2::pdep(x, mask)
}
