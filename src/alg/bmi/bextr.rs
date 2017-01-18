use int::Int;

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
pub fn bextr<T: Int>(source: T, start: T, length: T) -> T {
    (source >> start) & ((T::one() << length) - T::one())
}

pub trait BEXTR {
    fn bextr(self, Self, Self) -> Self;
}

impl<T: Int> BEXTR for T {
    fn bextr(self, start: Self, length: Self) -> Self {
        bextr(self, start, length)
    }
}
