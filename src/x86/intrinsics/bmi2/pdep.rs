use int::Int;
use alg;

#[cfg(RUSTC_IS_NIGHTLY)]
mod intrinsics {
    use int::Int;
    use std::mem::size_of;
    use alg;

    #[allow(dead_code)]
    extern "platform-intrinsic" {
        fn x86_bmi2_pdep_32(x: u32, y: u32) -> u32;
        fn x86_bmi2_pdep_64(x: u64, y: u64) -> u64;
    }

    #[inline] pub unsafe fn pdep<T: Int>(x: T, mask: T) -> T {
        match size_of::<T>() * 8 {
            32 => T::from_u32(x86_bmi2_pdep_32(x.to_u32(), mask.to_u32())),
            64 => T::from_u64(x86_bmi2_pdep_64(x.to_u64(), mask.to_u64())),
            _ => alg::x86::bmi2::pdep(x, mask),
        }
    }

} // mod intrinsics

/// Parallel bits deposit.
///
/// Scatter contiguous low order bits of `x` to the result at the positions
/// specified by the `mask_`.
///
/// All other bits (bits not set in the mask) of the result are set to zero.
///
/// **Keywords**: Parallel bits deposit, scatter bits.
///
/// # Assembly Instructions
///
/// - [`PDEP`](http://www.felixcloutier.com/x86/PDEP.html):
///   - Description: Parallel bits deposit.
///   - Architecture: x86.
///   - Instruction set: BMI2.
///   - Registers: 32/64 bit.
///
/// # Example
/// ```
/// use bitintr::x86::bmi2::*;
/// let n  = 0b1011_1110_1001_0011u16;
///
/// let m0 = 0b0110_0011_1000_0101u16;
/// let s0 = 0b0000_0010_0000_0101u16;
///
/// let m1 = 0b1110_1011_1110_1111u16;
/// let s1 = 0b1110_1001_0010_0011u16;
///
/// assert_eq!(pdep(n, m0), s0);
/// assert_eq!(n.pdep(m1), s1);
/// ```
#[cfg(RUSTC_IS_NIGHTLY)]
#[inline] pub fn pdep<T: Int>(x: T, mask: T) -> T {
    if cfg!(target_feature = "bmi2") {
        unsafe { intrinsics::pdep(x, mask) }
    } else {
        alg::x86::bmi2::pdep(x, mask)
    }
}
#[cfg(not(RUSTC_IS_NIGHTLY))]
#[inline] pub fn pdep<T: Int>(x: T, mask: T) -> T {
    alg::x86::bmi2::pdep(x, mask)
}

/// Method version of [`pdep`](fn.pdep.html).
pub trait PDEP {
    #[inline] fn pdep(self, Self) -> Self;
}

impl<T: Int> PDEP for T {
    #[inline] fn pdep(self, mask: Self) -> Self {
        pdep(self, mask)
    }
}
