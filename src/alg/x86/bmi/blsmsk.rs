use int::Int;

/// Get mask up to lowest set bit.
///
/// Sets all the bits of the result to `1` up to and including the lowest set
/// bit of `x`.
///
/// If `x` is zero, all the bits of the result are set.
///
/// # Assembly Instructions
///
/// - [`BLSMSK`](http://www.felixcloutier.com/x86/BLSMSK.html):
///   - Description: Get mask up to lowest set bit.
///   - Architecture: x86.
///   - Instruction set: BMI.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::bmi::*;
///
/// assert_eq!(blsmsk(0b0011_0000u8), 0b0001_1111u8);
/// assert_eq!(0b0000_0000u8.blsmsk(), 0b1111_1111u8);
/// ```
#[inline] pub fn blsmsk<T: Int>(x: T) -> T {
    x ^ (x.wrapping_sub(T::one()))
}

/// Method version of [`blsmsk`](fn.blsmsk.html).
pub trait BLSMSK {
    #[inline] fn blsmsk(self) -> Self;
}

impl<T: Int> BLSMSK for T {
    #[inline] fn blsmsk(self) -> Self {
        blsmsk(self)
    }
}
