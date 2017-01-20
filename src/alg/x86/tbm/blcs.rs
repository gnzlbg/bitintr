use int::Int;

/// Sets the least significant bit of `x`.
///
/// If there is no zero bit in `x`, it returns `x`.
///
/// # Assembly Instructions
///
/// - [`BLCS`](http://support.amd.com/TechDocs/24594.pdf):
///   - Description: Set lowest clear bit.
///   - Architecture: x86.
///   - Instruction set: TBM.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::tbm::*;
///
/// assert_eq!(blcs(0b0101_0001u8), 0b0101_0011u8);
/// assert_eq!(0b1111_1111u8.blcs(), 0b1111_1111u8);
/// ```
pub fn blcs<T: Int>(x: T) -> T {
    x | (x.wrapping_add(T::one()))
}

/// Method version of [`blcs`](fn.blcs.html).
pub trait BLCS {
    fn blcs(self) -> Self;
}

impl<T: Int> BLCS for T {
    fn blcs(self) -> T {
        blcs(self)
    }
}
