use int::IntF32T64;
use alg;
use x86;

pub trait BEXTRI {
    fn bextri(self, Self) -> Self;
}

macro_rules! alg_impl {
    ($T:ty) => (
        impl BEXTRI for $T {
            fn bextri(self, range: Self) -> Self {
                alg::bmi::bextri(self, range)
            }
        }
    )
}

alg_impl!(u8);
alg_impl!(u16);
alg_impl!(i8);
alg_impl!(i16);

impl<T: IntF32T64> BEXTRI for T {
    fn bextri(self, range: Self) -> Self {
        x86::bmi::bextri(self, range)
    }
}

/// Bit Field Extract (immediate form).
///
/// Extracts bits in `range` from the `source` to the least significant bits of
/// the result. Bits [7,0] of `range` specify the index to the first bit in the
/// range to be extracted, and bits [15,8] specify the length of the range.
///
/// Only bits up to `std::mem::size_of::<T>() - 1` are extracted.
///
/// The extracted bits are written in the result starting from the
/// least-significant bit. The high-order bits of the result are zeroed.
///
/// # Intrinsic (when available BMI 1)
///
/// [`BEXTR`](http://www.felixcloutier.com/x86/BEXTR.html): Bit field extract (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::bmi::bextri;
///
/// assert_eq!(bextri(0b0000_0000_0101_0000u16, 0b0000_0100_0000_0100u16), 0b0000_0000_0000_0101u16);
/// ```
pub fn bextri<T: BEXTRI>(x: T, range: T) -> T {
    BEXTRI::bextri(x, range)
}
