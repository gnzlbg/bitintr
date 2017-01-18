use int::IntF32T64;
use alg;
use x86;

pub trait BZHI {
    fn bzhi(self, Self) -> Self;
}

macro_rules! alg_impl {
    ($T:ty) => (
        impl BZHI for $T {
            fn bzhi(self, y: Self) -> Self {
                alg::bmi2::bzhi(self, y)
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
        x86::bmi2::bzhi(self, y)
    }
}

/// Zero the high bits of `x` at position >= `bit_position`.
///
/// # Panics
///
/// If `bit_position >= bit_size()`.
///
/// # Intrinsics (when available BMI2)
///
/// [`BZHI`](http://www.felixcloutier.com/x86/BZHI.html): Zero high bits
/// starting with specified bit position (supports 32/64 bit registers).
///
/// # Examples
///
/// ```
/// use bitintr::bmi2::bzhi;
///
/// let n = 0b1111_0010u32;
/// let s = 0b0001_0010u32;
/// assert_eq!(bzhi(n, 5), s);
/// ```
pub fn bzhi<T: BZHI>(x: T, y: T) -> T {
    BZHI::bzhi(x, y)
}
