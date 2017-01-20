use int::Int;

/// Counts the leading most significant bits set.
///
/// When all bits of the operand are set it returns the size of the operand in
/// bits.
///
/// **Keywords**: count leading sign bits, count leading ones, count leading
/// bits set.
///
/// # Assembly Instructions
///
/// - [`CLS`](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0487a.k_10775/index.html):
///   - Description: Count leading sign bits.
///   - Architecture: ARMv8.
///   - Registers: 32/64 bits.
///
/// # Example
///
/// ```
/// use bitintr::arm::v8::*;
///
/// let n = 0b1111_1111_1100_1010u16;
///
/// assert_eq!(n.cls(), 10);
/// assert_eq!(cls(0b1111_1111u8), 8);
/// ```
pub fn cls<T: Int>(x: T) -> T {
    T::leading_zeros(!x)
}

/// Method version of [`cls`](fn.cls.html).
pub trait CLS {
    fn cls(self) -> Self;
}

impl<T: Int> CLS for T {
    fn cls(self) -> T {
        cls(self)
    }
}
