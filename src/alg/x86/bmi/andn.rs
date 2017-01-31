use int::Int;

/// Bitwise logical `AND` of inverted `x` with `y`.
///
/// # Assembly Instructions
///
/// - [`ANDN`](http://www.felixcloutier.com/x86/ANDN.html):
///   - Description: Logical and not.
///   - Architecture: x86.
///   - Instruction set: BMI.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::bmi::*;
///
/// assert_eq!(andn(0, 0), 0);
/// assert_eq!(andn(0, 1), 1);
/// assert_eq!(andn(1, 0), 0);
/// assert_eq!(andn(1, 1), 0);
///
/// assert_eq!(andn(0b0000_0000u8, 0b0000_0000u8), 0b0000_0000u8);
/// assert_eq!(andn(0b0000_0000u8, 0b1111_1111u8), 0b1111_1111u8);
/// assert_eq!(andn(0b1111_1111u8, 0b0000_0000u8), 0b0000_0000u8);
/// assert_eq!(andn(0b1111_1111u8, 0b1111_1111u8), 0b0000_0000u8);
///
/// assert_eq!(andn(0b0100_0000u8, 0b0101_1101u8), 0b0001_1101u8);
/// assert_eq!(0b0100_0000u8.andn(0b0101_1101u8), 0b0001_1101u8);
/// ```
#[inline]
pub fn andn<T: Int>(x: T, y: T) -> T {
    !x & y
}

/// Method version of [`andn`](fn.andn.html).
pub trait ANDN {
    #[inline]
    fn andn(self, Self) -> Self;
}

impl<T: Int> ANDN for T {
    #[inline]
    fn andn(self, y: Self) -> Self {
        andn(self, y)
    }
}
