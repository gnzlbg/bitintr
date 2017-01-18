use int::IntF32T64;
use alg;
use x86;

pub trait PEXT {
    fn pext(self, Self) -> Self;
}

macro_rules! alg_impl {
    ($T:ty) => (
        impl PEXT for $T {
            fn pext(self, y: Self) -> Self {
                alg::bmi2::pext(self, y)
            }
        }
    )
}

alg_impl!(u8);
alg_impl!(u16);
alg_impl!(i8);
alg_impl!(i16);

impl<T: IntF32T64> PEXT for T {
    fn pext(self, y: Self) -> Self {
        x86::bmi2::pext(self, y)
    }
}

/// Parallel bits extract.
///
/// Gathers the bits of `x` specified by the `mask_` into the contiguous low
/// order bit positions of the result.
///
/// The remaining high-order bits of the result are set to zero.
///
/// **Keywords**: Parallel bits extract, gather bits.
///
/// # Intrinsics (when available BMI2)
///
/// [`PEXT`](http://www.felixcloutier.com/x86/PEXT.html): Parallel bits extract
/// (supports 32/64 bit registers).
///
/// # Examples
/// ```
/// use bitintr::bmi2::pext;
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
/// assert_eq!(pext(n, m1), s1);
/// ```
pub fn pext<T: PEXT>(x: T, y: T) -> T {
    PEXT::pext(x, y)
}
