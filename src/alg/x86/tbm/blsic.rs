use int::Int;

/// Clears least significant bit and sets all other bits.
///
/// If there is no set bit in `x`, it sets all the bits.
///
/// # Assembly Instructions
///
/// - [`BLSIC`](http://support.amd.com/TechDocs/24594.pdf):
///   - Description: Isolate lowest set bit and complement.
///   - Architecture: x86.
///   - Instruction set: TBM.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::tbm::*;
///
/// assert_eq!(blsic(0b0101_0100u8), 0b1111_1011u8);
/// assert_eq!(0b0000_0000u8.blsic(), 0b1111_1111u8);
/// ```
#[inline]
pub fn blsic<T: Int>(x: T) -> T {
    !x | (x.wrapping_sub(T::one()))
}

/// Method version of [`blsic`](fn.blsic.html).
pub trait BLSIC {
    #[inline]
    fn blsic(self) -> Self;
}

impl<T: Int> BLSIC for T {
    #[inline]
    fn blsic(self) -> T {
        blsic(self)
    }
}
