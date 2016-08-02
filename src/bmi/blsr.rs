use int::Int;
use alg;

pub trait BLSR {
    fn blsr(self) -> Self;
}

impl<T: Int> BLSR for T {
    fn blsr(self) -> Self {
        alg::bmi::blsr(self)
    }
}

/// Resets the lowest set bit of `x`.
///
/// # Panics
///
/// If `x` is zero (because wrapping unsigned arithmetic on primitive types
/// `panics!`).
///
/// TODO: It would probably better to define this as undefined behavior, and let
/// a higher-level abstraction decide whether it wants to panic or not.
///
/// # Intrinsic (when available BMI1)
///
/// [`BLSR`](http://www.felixcloutier.com/x86/BLSR.html): Reset lowest set bit
/// (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blsr<T: BLSR>(x: T) -> T {
    BLSR::blsr(x)
}
