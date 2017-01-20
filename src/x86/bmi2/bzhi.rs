use int::IntF32T64;
use alg;
use x86;

/// Method version of [`bzhi`](fn.bzhi.html).
pub trait BZHI {
    fn bzhi(self, Self) -> Self;
}

macro_rules! alg_impl {
    ($T:ty) => (
        impl BZHI for $T {
            fn bzhi(self, y: Self) -> Self {
                alg::x86::bmi2::bzhi(self, y)
            }
        }
    )
}

alg_impl!(u8);
alg_impl!(u16);
alg_impl!(i8);
alg_impl!(i16);

impl<T: IntF32T64> BZHI for T {
    fn bzhi(self, y: Self) -> Self {
        x86::intrinsics::bmi2::bzhi(self, y)
    }
}

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
pub fn bzhi<T: BZHI>(x: T, y: T) -> T {
    BZHI::bzhi(x, y)
}
