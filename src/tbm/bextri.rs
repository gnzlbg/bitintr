use int::IntF32T64;
use alg;
use x86;

pub trait BEXTRI {
    fn bextri(self, start: Self, length: Self) -> Self;
}

macro_rules! alg_impl {
    ($T:ty) => (
        impl BEXTRI for $T {
            fn bextri(self, start: Self, length: Self) -> Self {
                alg::tbm::bextri(self, start, length)
            }
        }
    )
}

alg_impl!(u8);
alg_impl!(u16);
alg_impl!(i8);
alg_impl!(i16);

impl<T: IntF32T64> BEXTRI for T {
    fn bextri(self, start: T, length: T) -> T {
        x86::bmi::bextr(self, start, length)
    }
}

/// Bit Field Extract (immediate form).
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
/// # Intrinsic (when available TBM)
///
/// [`BEXTRI`](http://support.amd.com/TechDocs/24592.pdf): Bit field extract
/// (immediate, supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::tbm::bextri;
///
/// assert_eq!(bextri(0b0101_0000u8, 4, 4), 0b0000_0101u8);
/// ```
pub fn bextri<T: BEXTRI>(x: T, y: T, z: T) -> T {
    BEXTRI::bextri(x, y, z)
}
