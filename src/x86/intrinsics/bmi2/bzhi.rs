use int::Int;
use alg;

#[cfg(RUSTC_IS_NIGHTLY)]
mod intrinsics {
    use alg;
    use int::Int;
    use std::mem::size_of;

    #[allow(dead_code)]
    extern "platform-intrinsic" {
        fn x86_bmi2_bzhi_32(x: u32, y: u32) -> u32;
        fn x86_bmi2_bzhi_64(x: u64, y: u64) -> u64;
    }

    #[inline] pub unsafe fn bzhi<T: Int>(x: T, y: T) -> T {
        match size_of::<T>() * 8 {
            32 => T::from_u32(x86_bmi2_bzhi_32(x.to_u32(), y.to_u32())),
            64 => T::from_u64(x86_bmi2_bzhi_64(x.to_u64(), y.to_u64())),
            _ => alg::x86::bmi2::bzhi(x, y),
        }
    }

} // mod intrinsics

/// Zero the high bits of `x` at position >= `bit_position`.
///
/// # Panics
///
/// If `bit_position >= bit_size()` the behavior is undefined (panics in debug
/// builds).
///
/// # Assembly Instructions
///
/// - [`BZHI`](http://www.felixcloutier.com/x86/BZHI.html):
///   - Description: Zero high bits starting with specified bit position.
///   - Architecture: x86.
///   - Instruction set: BMI2.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::bmi2::*;
///
/// let n = 0b1111_0010u32;
/// let s = 0b0001_0010u32;
/// assert_eq!(bzhi(n, 5), s);
/// assert_eq!(n.bzhi(5), s);
/// ```
#[cfg(RUSTC_IS_NIGHTLY)]
#[inline] pub fn bzhi<T: Int>(x: T, bit_position: T) -> T {
    if cfg!(target_feature = "bmi2") {
        debug_assert!(bit_position < T::bit_size());
        unsafe { intrinsics::bzhi(x, bit_position) }
    } else {
        alg::x86::bmi2::bzhi(x, bit_position)
    }
}
#[cfg(not(RUSTC_IS_NIGHTLY))]
#[inline] pub fn bzhi<T: Int>(x: T, bit_position: T) -> T {
    alg::x86::bmi2::bzhi(x, bit_position)
}

/// Method version of [`bzhi`](fn.bzhi.html).
pub trait BZHI {
    #[inline] fn bzhi(self, Self) -> Self;
}

impl<T: Int> BZHI for T {
    #[inline] fn bzhi(self, y: Self) -> Self {
        bzhi(self, y)
    }
}
