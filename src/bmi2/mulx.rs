use int::{Int, IntF32T64};
use alg;
use x86;

pub trait MULX {
    fn mulx(self, Self, &mut Self) -> Self;
}

impl<T: Int + alg::bmi2::MULX> MULX for T {
    default fn mulx(self, y: Self, hi: &mut Self) -> Self {
        alg::bmi2::mulx(self, y, hi)
    }
}

impl<T: IntF32T64 + alg::bmi2::MULX> MULX for T {
    fn mulx(self, y: Self, hi: &mut Self) -> Self {
        x86::bmi2::mulx(self, y, hi)
    }
}

/// Unsigned multiplication of `x` with `y` storing the high half of the result
/// in `p` and returning the low half of the result.
///
/// # Intrinsics (when available BMI2)
///
/// [`MULX`](http://www.felixcloutier.com/x86/MULX.html): Unsigned Multiply
/// Without Affecting Flags (supports 32/64 bit registers).
///
/// # Examples
///
/// ```
/// assert!(false);
/// ```
pub fn mulx<T: MULX + alg::bmi2::MULX>(x: T, y: T, hi: &mut T) -> T {
    MULX::mulx(x, y, hi)
}
