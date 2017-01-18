use int::Int;

/// Get mask up to lowest set bit.
///
/// Sets all the bits of the result to `1` up to and including the lowest set
/// bit of `x`.
///
/// If `x` is zero, all the bits of the result are set.
///
/// # Intrinsic (when available BMI1)
///
/// [`BLSMSK`](http://www.felixcloutier.com/x86/BLSMSK.html): Get mask up to
/// lowest set bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::bmi::blsmsk;
///
/// assert_eq!(blsmsk(0b0011_0000u8), 0b0001_1111u8);
/// assert_eq!(blsmsk(0b0000_0000u8), 0b1111_1111u8);
/// ```
pub fn blsmsk<T: Int>(x: T) -> T {
    x ^ (x.wrapping_sub(T::one()))
}

pub trait BLSMSK {
    fn blsmsk(self) -> Self;
}

impl<T: Int> BLSMSK for T {
    fn blsmsk(self) -> Self {
        blsmsk(self)
    }
}
