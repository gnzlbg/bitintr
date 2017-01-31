use int::Int;
use alg;

#[cfg(RUSTC_IS_NIGHTLY)]
mod intrinsics {
    use int::Int;
    use std::mem::size_of;
    use alg;

    #[allow(dead_code)]
    extern "platform-intrinsic" {
        fn x86_bmi_bextr_32(x: u32, y: u32) -> u32;
        fn x86_bmi_bextr_64(x: u64, y: u64) -> u64;
    }

    #[inline]
    pub unsafe fn bextr<T: Int>(source: T, start: T, length: T) -> T {
        match size_of::<T>() * 8 {
            32 => {
                T::from_u32(x86_bmi_bextr_32(source.to_u32(),
                                             ((start.to_u32() & 0xffu32) |
                                              ((length.to_u32() & 0xffu32) << 8u32))))
            }
            64 => {
                T::from_u64(x86_bmi_bextr_64(source.to_u64(),
                                             ((start.to_u64() & 0xffu64) |
                                              ((length.to_u64() & 0xffu64) << 8u64))))
            }
            _ => alg::x86::bmi::bextr(source, start, length),
        }
    }


} // mod intrinsics

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
/// # Assembly Instructions
///
/// - [`BEXTR`](http://www.felixcloutier.com/x86/BEXTR.html):
///   - Description: Bit field extract.
///   - Architecture: x86.
///   - Instruction set: BMI.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::bmi::*;
///
/// assert_eq!(bextr(0b0101_0000u8, 4, 4), 0b0000_0101u8);
/// assert_eq!(0b0101_0000u8.bextr(4, 4), 0b0000_0101u8);
/// ```
#[cfg(RUSTC_IS_NIGHTLY)]
#[inline]
pub fn bextr<T: Int>(source: T, start: T, length: T) -> T {
    if cfg!(target_feature = "bmi2") {
        unsafe { intrinsics::bextr(source, start, length) }
    } else {
        alg::x86::bmi::bextr(source, start, length)
    }
}
#[cfg(not(RUSTC_IS_NIGHTLY))]
#[inline]
pub fn bextr<T: Int>(source: T, start: T, length: T) -> T {
    alg::x86::bmi::bextr(source, start, length)
}

/// Method version of [`bextr`](fn.bextr.html).
pub trait BEXTR {
    #[inline]
    fn bextr(self, Self, Self) -> Self;
}

impl<T: Int> BEXTR for T {
    #[inline]
    fn bextr(self, start: Self, length: Self) -> Self {
        bextr(self, start, length)
    }
}
