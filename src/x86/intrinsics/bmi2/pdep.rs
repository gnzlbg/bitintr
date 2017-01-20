use int::IntF32T64;
use alg;

#[cfg(RUSTC_IS_NIGHTLY)]
mod intrinsics {
    use int::IntF32T64;
    use std::mem::size_of;

    #[allow(dead_code)]
    extern "platform-intrinsic" {
        fn x86_bmi2_pdep_32(x: u32, y: u32) -> u32;
        fn x86_bmi2_pdep_64(x: u64, y: u64) -> u64;
    }

    pub unsafe fn pdep<T: IntF32T64>(x: T, y: T) -> T {
        match size_of::<T>() * 8 {
            32 => T::from_u32(x86_bmi2_pdep_32(x.to_u32(), y.to_u32())),
            64 => T::from_u64(x86_bmi2_pdep_64(x.to_u64(), y.to_u64())),
            _ => unreachable!(),
        }
    }


} // mod intrinsics

#[cfg(RUSTC_IS_NIGHTLY)]
pub fn pdep<T: IntF32T64>(x: T, mask_: T) -> T {
    if cfg!(target_feature = "bmi2") {
        unsafe { intrinsics::pdep(x, mask_) }
    } else {
        alg::x86::bmi2::pdep(x, mask_)
    }
}

#[cfg(not(RUSTC_IS_NIGHTLY))]
pub fn pdep<T: IntF32T64>(x: T, mask_: T) -> T {
    alg::x86::bmi2::pdep(x, mask_)
}
