use int::IntF32T64;
use alg;

#[cfg(RUSTC_IS_NIGHTLY)]
mod intrinsics {
    use int::IntF32T64;
    use std::mem::size_of;
    use x86::bmi::bextri::bextri;

    pub unsafe fn bextr<T: IntF32T64>(source: T, start: T, length: T) -> T {
        match size_of::<T>() * 8 {
            32 => {
                bextri(source,
                       ((start & T::from_u32(0xff)) |
                        ((length & T::from_u32(0xff)) << T::from_u32(8))))
            }
            64 => {
                bextri(source,
                       ((start & T::from_u64(0xff)) |
                        ((length & T::from_u64(0xff)) << T::from_u64(8))))
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
        alg::bmi::bextr(source, start, length)
    }
}

#[cfg(not(RUSTC_IS_NIGHTLY))]
pub fn bextr<T: IntF32T64>(source: T, start: T, length: T) -> T {
    alg::bmi::bextr(source, start, length)
}
