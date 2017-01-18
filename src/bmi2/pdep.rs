use int::IntF32T64;
use alg;
use x86;

pub trait PDEP {
    fn pdep(self, Self) -> Self;
}

macro_rules! alg_impl {
    ($T:ty) => (
        impl PDEP for $T {
            fn pdep(self, y: Self) -> Self {
                alg::bmi2::pdep(self, y)
            }
        }
    )
}

alg_impl!(u8);
alg_impl!(u16);
alg_impl!(i8);
alg_impl!(i16);


impl<T: IntF32T64> PDEP for T {
    fn pdep(self, y: Self) -> Self {
        x86::bmi2::pdep(self, y)
    }
}

/// Parallel bits deposit.
///
/// Scatter contiguous low order bits of `x` to the result at the positions
/// specified by the `mask_`.
///
/// All other bits (bits not set in the mask) of the result are set to zero.
///
/// **Keywords**: Parallel bits deposit, scatter bits.
///
/// # Intrinsics (when available BMI2)
///
/// [`PDEP`](http://www.felixcloutier.com/x86/PDEP.html): Parallel bits deposit
/// (supports 32/64 bit registers).
///
/// # Examples
/// ```
/// use bitintr::bmi2::pdep;
/// let n  = 0b1011_1110_1001_0011u16;
///
/// let m0 = 0b0110_0011_1000_0101u16;
/// let s0 = 0b0000_0010_0000_0101u16;
///
/// let m1 = 0b1110_1011_1110_1111u16;
/// let s1 = 0b1110_1001_0010_0011u16;
///
/// assert_eq!(pdep(n, m0), s0);
/// assert_eq!(pdep(n, m1), s1);
/// ```
pub fn pdep<T: PDEP>(x: T, y: T) -> T {
    PDEP::pdep(x, y)
}
