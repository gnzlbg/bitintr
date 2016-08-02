use int::Int;
use alg;

pub trait ANDN {
    fn andn(self, Self) -> Self;
}

impl<T: Int> ANDN for T {
    fn andn(self, y: Self) -> Self {
        alg::bmi::andn(self, y)
    }
}

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
/// assert!(false);
/// ```
pub fn andn<T: ANDN>(x: T, y: T) -> T {
    ANDN::andn(x, y)
}
