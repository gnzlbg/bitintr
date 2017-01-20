use int::IntF32T64;
use alg;
use x86;

/// Method version of [`bextr`](fn.bextr.html).
pub trait BEXTR {
    fn bextr(self, range: Self) -> Self;
}

macro_rules! alg_impl {
    ($T:ty) => (
        impl BEXTR for $T {
            fn bextr(self, range: Self) -> Self {
                alg::x86::tbm::bextr(self, range)
            }
        }
    )
}

alg_impl!(u8);
alg_impl!(u16);
alg_impl!(i8);
alg_impl!(i16);

impl<T: IntF32T64> BEXTR for T {
    fn bextr(self, range: T) -> T {
        x86::intrinsics::bmi::bextr(self, range & T::from_u32(0xff), range >> T::from_u32(8))
    }
}

/// Bit Field Extract.
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
/// # Assembly Instructions
///
/// - [`BEXTR`](https://support.amd.com/TechDocs/24594.pdf):
///   - Description: Bit field extract.
///   - Architecture: x86.
///   - Instruction set: TBM.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::tbm::*;
///
/// assert_eq!(bextr(0b0000_0000_0101_0000u16, 0b0000_0100_0000_0100u16), 0b0000_0000_0000_0101u16);
/// assert_eq!(0b0000_0000_0101_0000u16.bextr(0b0000_0100_0000_0100u16), 0b0000_0000_0000_0101u16);
/// ```
pub fn bextr<T: BEXTR>(source: T, range: T) -> T {
    BEXTR::bextr(source, range)
}
