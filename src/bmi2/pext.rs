use int::{Int, IntF32T64};
use alg;
use x86;

pub trait PEXT {
    fn pext(self, Self) -> Self;
}

impl<T: Int> PEXT for T {
    default fn pext(self, y: Self) -> Self {
        alg::bmi2::pext(self, y)
    }
}

impl<T: IntF32T64> PEXT for T {
    fn pext(self, y: Self) -> Self {
        x86::bmi2::pext(self, y)
    }
}

/// Gathers the bits of `x` specified by the `mask_` into the contiguous low
/// order bit positions of the result.
///
/// The remaining high-order bits of the result are set to zero.
///
/// **Keywords**: Parallel bits extract, gather bits.
///
/// # Intrinsics (when available BMI2)
///
/// [`PEXT`](http://www.felixcloutier.com/x86/PEXT.html): Parallel bits extract
/// (supports 32/64 bit registers).
///
/// # Examples
/// ```
/// assert!(false);
/// ```
pub fn pext<T: PEXT>(x: T, y: T) -> T {
    PEXT::pext(x, y)
}
