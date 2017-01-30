use int::Int;

/// Reverse the order of the bytes.
///
/// **Keywords**: count leading sign bits, count leading ones, count leading
/// bits set.
///
/// # Assembly Instructions
///
/// - [`REV`](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0419c/index.html):
///   - Description: Reverses the byte order in a register.
///   - Architecture: ARMv6, ARMv7, ARMv8.
///   - Registers: 32 (v6, v7)/64 (v8) bits.
///
/// # Example
///
/// ```
/// use bitintr::arm::v6::*;
///
/// let n = 0b1111_1111_1100_1010u16;
/// let m = 0b1100_1010_1111_1111u16;
///
/// assert_eq!(n.rev(), m);
/// assert_eq!(m.rev(), n);
/// ```
#[inline] pub fn rev<T: Int>(x: T) -> T {
    x.swap_bytes()
}

/// Method version of [`rev`](fn.rev.html).
pub trait REV {
    #[inline] fn rev(self) -> Self;
}

impl<T: Int> REV for T {
    #[inline] fn rev(self) -> T {
        rev(self)
    }
}
