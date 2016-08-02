use int::Int;
use alg;

pub trait BLSMSK {
    fn blsmsk(self) -> Self;
}

impl<T: Int> BLSMSK for T {
    fn blsmsk(self) -> Self {
        alg::bmi::blsmsk(self)
    }
}

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
/// assert!(false);
/// ```
pub fn blsmsk<T: BLSMSK>(x: T) -> T {
    BLSMSK::blsmsk(x)
}
