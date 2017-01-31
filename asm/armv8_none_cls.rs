extern crate bitintr;

#[no_mangle]
pub fn cls_u32(x: u32) -> u32 {
    bitintr::arm::v8::cls(x)
}

#[no_mangle]
pub fn cls_u64(x: u64) -> u64 {
    bitintr::arm::v8::cls(x)
}
