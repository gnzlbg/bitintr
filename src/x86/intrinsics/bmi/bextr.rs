use int::IntF32T64;
use alg;

#[cfg(RUSTC_IS_NIGHTLY)]
mod intrinsics {
    use int::IntF32T64;
    use std::mem::size_of;

    #[allow(dead_code)]
    extern "platform-intrinsic" {
        fn x86_bmi_bextr_32(x: u32, y: u32) -> u32;
        fn x86_bmi_bextr_64(x: u64, y: u64) -> u64;
    }

    pub unsafe fn bextr<T: IntF32T64>(source: T, start: T, length: T) -> T {
        match size_of::<T>() * 8 {
            32 => {
                T::from_u32(x86_bmi_bextr_32(source.to_u32(),
                                 ((start.to_u32() & 0xffu32) |
                                  ((length.to_u32() & 0xffu32) << 8u32))))
            }
            64 => {
                T::from_u64(x86_bmi_bextr_64(source.to_u64(),
                                 ((start.to_u64() & 0xffu64) |
                                  ((length.to_u64() & 0xffu64) << 8u64))))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(RUSTC_IS_NIGHTLY)]
pub fn bextr<T: IntF32T64>(source: T, start: T, length: T) -> T {
    if cfg!(target_feature = "bmi") {
        unsafe { intrinsics::bextr(source, start, length) }
    } else {
        alg::x86::bmi::bextr(source, start, length)
    }
}

#[cfg(not(RUSTC_IS_NIGHTLY))]
pub fn bextr<T: IntF32T64>(source: T, start: T, length: T) -> T {
    alg::x86::bmi::bextr(source, start, length)
}
