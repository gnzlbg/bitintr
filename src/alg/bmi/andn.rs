use int::Int;

/// Bitwise logical `AND` of inverted `x` with `y`.
///
/// # Intrinsic (when available BMI1)
///
/// [`ANDN`](http://www.felixcloutier.com/x86/ANDN.html): Logical
/// and not (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::bmi::andn;
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
/// ```
pub fn andn<T: Int>(x: T, y: T) -> T {
    !x & y
}

pub trait ANDN {
    fn andn(self, Self) -> Self;
}

impl<T: Int> ANDN for T {
    fn andn(self, y: Self) -> Self {
        andn(self, y)
    }
}
