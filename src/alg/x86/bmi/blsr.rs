use int::Int;

/// Resets the lowest set bit of `x`.
///
/// # Panics
///
/// If `x` is zero the behavior is undefined (panics in debug builds).
///
/// # Assembly Instructions
///
/// - [`BLSR`](http://www.felixcloutier.com/x86/BLSR.html):
///   - Description: Reset lowest set bit.
///   - Architecture: x86.
///   - Instruction set: BMI.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::bmi::*;
///
/// assert_eq!(blsr(0b0011_0000u8), 0b0010_0000u8);
/// assert_eq!(0b0011_0000u8.blsr(), 0b0010_0000u8);
/// ```
#[inline]
pub fn blsr<T: Int>(x: T) -> T {
    debug_assert!(x != T::zero());
    x & (x.wrapping_sub(T::one()))
}

/// Method version of [`blsr`](fn.blsr.html).
pub trait BLSR {
    #[inline]
    fn blsr(self) -> Self;
}

impl<T: Int> BLSR for T {
    #[inline]
    fn blsr(self) -> Self {
        blsr(self)
    }
}
