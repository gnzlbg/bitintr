use int::{Int, IntF32T64};
use alg;
use x86;

pub trait PDEP {
    fn pdep(self, Self) -> Self;
}

impl<T: Int> PDEP for T {
    default fn pdep(self, y: Self) -> Self {
        alg::bmi2::pdep(self, y)
    }
}

impl<T: IntF32T64> PDEP for T {
    fn pdep(self, y: Self) -> Self {
        x86::bmi2::pdep(self, y)
    }
}

/// Scatter contiguous low order bits of `x` to the result at the positions
/// specified by the `mask_`.
///
/// All other bits (bits not set in the mask) of the result are set to zero.
///
/// **Keywords**: Parallel bits deposit, scatter bits.
///
/// # Intrinsics (when available BMI2)
///
/// [`PDEP`](http://www.felixcloutier.com/x86/PDEP.html): Parallel bits deposit
/// (supports 32/64 bit registers).
///
/// # Examples
/// ```
/// assert!(false);
/// ```
pub fn pdep<T: PDEP>(x: T, y: T) -> T {
    PDEP::pdep(x, y)
}
