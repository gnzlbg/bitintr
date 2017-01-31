use int::Int;

/// Extract lowest set isolated bit.
///
/// Extracts the lowest set bit of `x` and sets the corresponding bit in the
/// result (all other bits of the result are zeroed).
///
/// # Assembly Instructions
///
/// - [`BLSI`](http://www.felixcloutier.com/x86/BLSI.html):
///   - Description: Extract lowest set isolated bit.
///   - Architecture: x86.
///   - Instruction set: BMI.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::bmi::*;
///
/// assert_eq!(blsi(0b1101_0000u8), 0b0001_0000u8);
/// assert_eq!(0b0100_1000u8.blsi(), 0b0000_1000u8);
/// ```
#[inline]
pub fn blsi<T: Int>(x: T) -> T {
    x & x.wrapping_neg()
}

/// Method version of [`blsi`](fn.blsi.html).
pub trait BLSI {
    #[inline]
    fn blsi(self) -> Self;
}

impl<T: Int> BLSI for T {
    #[inline]
    fn blsi(self) -> Self {
        blsi(self)
    }
}
