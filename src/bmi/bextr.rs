use int::IntF32T64;
use alg;
use x86;

pub trait BEXTR {
    fn bextr(self, Self, Self) -> Self;
}

macro_rules! alg_impl {
    ($T:ty) => (
        impl BEXTR for $T {
            fn bextr(self, start: Self, length: Self) -> Self {
                alg::bmi::bextr(self, start, length)
            }
        }
    )
}

alg_impl!(u8);
alg_impl!(u16);
alg_impl!(i8);
alg_impl!(i16);

impl<T: IntF32T64> BEXTR for T {
    fn bextr(self, start: Self, length: Self) -> Self {
        x86::bmi::bextr(self, start, length)
    }
}

/// Bit Field Extract.
///
/// Extracts bits in range [`start`, `start` + `length`) from the `source` to
/// the least significant bits of the result.
///
/// Bits [7,0] of `range` specify the index to the first bit in the range to be
/// extracted, and bits [15,8] specify the length of the range.
///
/// Only bits up to `std::mem::size_of::<T>() - 1` are extracted.
///
/// The extracted bits are written in the result starting from the
/// least-significant bit. The high-order bits of the result are zeroed.
///
/// # Intrinsic (when available BMI1)
///
/// - [`BEXTR`](http://www.felixcloutier.com/x86/BEXTR.html):
///   Bit field extract (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::bmi::bextr;
/// 
/// assert_eq!(bextr(0b0101_0000u8, 4, 4), 0b0000_0101u8);
/// ```
pub fn bextr<T: BEXTR>(x: T, y: T, z: T) -> T {
    BEXTR::bextr(x, y, z)
}
