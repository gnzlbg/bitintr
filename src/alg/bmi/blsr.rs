use int::Int;

/// Resets the lowest set bit of `x`.
///
/// # Panics
///
/// If `x` is zero the behavior is undefined (panics in debug builds).
///
/// # Intrinsic (when available BMI1)
///
/// [`BLSR`](http://www.felixcloutier.com/x86/BLSR.html): Reset lowest set bit
/// (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::bmi::blsr;
/// 
/// assert_eq!(blsr(0b0011_0000u8), 0b0010_0000u8);
/// ```
pub fn blsr<T: Int>(x: T) -> T {
    debug_assert!(x != T::zero());
    x & (x.wrapping_sub(T::one()))
}

pub trait BLSR {
    fn blsr(self) -> Self;
}

impl<T: Int> BLSR for T {
    fn blsr(self) -> Self {
        blsr(self)
    }
}
