use int::IntF32T64;
use alg;

mod intrinsics {
    use int::IntF32T64;
    use std::mem::size_of;

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn x86_bmi2_bzhi_32(x: u32, y: u32) -> u32;
    fn x86_bmi2_bzhi_64(x: u64, y: u64) -> u64;
}

pub unsafe fn bzhi<T: IntF32T64>(x: T, y: T) -> T {
    match size_of::<T>() * 8 {
        32 => T::from_u32(x86_bmi2_bzhi_32(x.to_u32(), y.to_u32())),
        64 => T::from_u64(x86_bmi2_bzhi_64(x.to_u64(), y.to_u64())),
        _ => unreachable!(),
    }
}

}  // mod intrinsics

pub fn bzhi<T: IntF32T64>(x: T, bit_position: T) -> T {
    debug_assert!(bit_position < T::bit_size());
    if cfg!(target_feature = "bmi2") {
        unsafe { intrinsics::bzhi(x, bit_position) }
    } else {
        alg::bmi2::bzhi(x, bit_position)
    }
}
