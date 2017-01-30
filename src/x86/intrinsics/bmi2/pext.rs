use int::Int;
use alg;

#[cfg(RUSTC_IS_NIGHTLY)]
mod intrinsics {
    use int::Int;
    use std::mem::size_of;
    use alg;

    #[allow(dead_code)]
    extern "platform-intrinsic" {
        fn x86_bmi2_pext_32(x: u32, y: u32) -> u32;
        fn x86_bmi2_pext_64(x: u64, y: u64) -> u64;
    }

    pub unsafe fn pext<T: Int>(x: T, mask: T) -> T {
        match size_of::<T>() * 8 {
            32 => T::from_u32(x86_bmi2_pext_32(x.to_u32(), mask.to_u32())),
            64 => T::from_u64(x86_bmi2_pext_64(x.to_u64(), mask.to_u64())),
            _ => alg::x86::bmi2::pext(x, mask),
        }
    }

} // mod intrinsics

/// Parallel bits extract.
///
/// Gathers the bits of `x` specified by the `mask_` into the contiguous low
/// order bit positions of the result.
///
/// The remaining high-order bits of the result are set to zero.
///
/// **Keywords**: Parallel bits extract, gather bits.
///
/// # Assembly Instructions
///
/// - [`PEXT`](http://www.felixcloutier.com/x86/PEXT.html):
///   - Description: Parallel bits extract.
///   - Architecture: x86.
///   - Instruction set: BMI2.
///   - Registers: 32/64 bit.
///
/// # Example
/// ```
/// use bitintr::x86::bmi2::*;
///
/// let n  = 0b1011_1110_1001_0011u16;
///
/// let m0 = 0b0110_0011_1000_0101u16;
/// let s0 = 0b0000_0000_0011_0101u16;
///
/// let m1 = 0b1110_1011_1110_1111u16;
/// let s1 = 0b0001_0111_0100_0011u16;
///
/// assert_eq!(pext(n, m0), s0);
/// assert_eq!(n.pext(m1), s1);
/// ```
#[cfg(RUSTC_IS_NIGHTLY)]
pub fn pext<T: Int>(x: T, mask: T) -> T {
    if cfg!(target_feature = "bmi2") {
        unsafe { intrinsics::pext(x, mask) }
    } else {
        alg::x86::bmi2::pext(x, mask)
    }
}
#[cfg(not(RUSTC_IS_NIGHTLY))]
pub fn pext<T: Int>(x: T, mask: T) -> T {
    alg::x86::bmi2::pext(x, mask)
}

/// Method version of [`pext`](fn.pext.html).
pub trait PEXT {
    fn pext(self, Self) -> Self;
}

impl<T: Int> PEXT for T {
    fn pext(self, mask: Self) -> Self {
        pext(self, mask)
    }
}
