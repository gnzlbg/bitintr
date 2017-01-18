use int::IntF32T64;
use alg;

mod intrinsics {

    use int::IntF32T64;
    use std::mem::size_of;

    #[allow(dead_code)]
    extern "platform-intrinsic" {
        fn x86_bmi2_pext_32(x: u32, y: u32) -> u32;
        fn x86_bmi2_pext_64(x: u64, y: u64) -> u64;
    }

    pub unsafe fn pext<T: IntF32T64>(x: T, y: T) -> T {
        match size_of::<T>() * 8 {
            32 => T::from_u32(x86_bmi2_pext_32(x.to_u32(), y.to_u32())),
            64 => T::from_u64(x86_bmi2_pext_64(x.to_u64(), y.to_u64())),
            _ => unreachable!(),
        }
    }

} // mod intrinsics

pub fn pext<T: IntF32T64>(x: T, mask_: T) -> T {
    if cfg!(target_feature = "bmi2") {
        unsafe { intrinsics::pext(x, mask_) }
    } else {
        alg::bmi2::pext(x, mask_)
    }
}
