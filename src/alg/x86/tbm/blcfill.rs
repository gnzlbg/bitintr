use int::Int;

/// Clears all bits below the least significant zero bit of `x`.
///
/// If there is no zero bit in `x`, it returns zero.
///
/// # Assembly Instructions
///
/// - [`BLCFILL`](http://support.amd.com/TechDocs/24594.pdf):
///   - Description: Fill from lowest clear bit.
///   - Architecture: x86.
///   - Instruction set: TBM.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::tbm::*;
///
/// assert_eq!(blcfill(0b0101_0111u8), 0b0101_0000u8);
/// assert_eq!(0b1111_1111u8.blcfill(), 0u8);
/// ```
#[inline] pub fn blcfill<T: Int>(x: T) -> T {
    x & (x.wrapping_add(T::one()))
}

/// Method version of [`blcfill`](fn.blcfill.html).
pub trait BLCFILL {
    #[inline] fn blcfill(self) -> Self;
}

impl<T: Int> BLCFILL for T {
    #[inline] fn blcfill(self) -> T {
        blcfill(self)
    }
}
