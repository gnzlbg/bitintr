use int::Int;

/// Sets all bits of `x` below the least significant one.
///
/// If there is no set bit in `x`, it sets all the bits.
///
/// # Assembly Instructions
///
/// - [`BLSFILL`](http://support.amd.com/TechDocs/24594.pdf):
///   - Description: Fill from lowest set bit.
///   - Architecture: x86.
///   - Instruction set: TBM.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::tbm::*;
///
/// assert_eq!(blsfill(0b0101_0100u8), 0b0101_0111u8);
/// assert_eq!(0b0000_0000u8.blsfill(), 0b1111_1111u8);
/// ```
#[inline]
pub fn blsfill<T: Int>(x: T) -> T {
    x | (x.wrapping_sub(T::one()))
}

/// Method version of [`blsfill`](fn.blsfill.html).
pub trait BLSFILL {
    #[inline]
    fn blsfill(self) -> Self;
}

impl<T: Int> BLSFILL for T {
    #[inline]
    fn blsfill(self) -> T {
        blsfill(self)
    }
}
