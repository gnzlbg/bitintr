use int::Int;
use alg;

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
pub fn bextri<T: Int>(source: T, range: T) -> T {
    alg::bmi::bextr(source, range & T::from_u32(0xff), range >> T::from_u32(8))
}

pub trait BEXTRI {
    fn bextri(self, Self) -> Self;
}

impl<T: Int> BEXTRI for T {
    fn bextri(self, range: Self) -> Self {
        bextri(self, range)
    }
}
