use int::Int;
use alg;

pub trait BLSI {
    fn blsi(self) -> Self;
}

impl<T: Int> BLSI for T {
    fn blsi(self) -> Self {
        alg::bmi::blsi(self)
    }
}

/// Extracts the lowest set bit of `x` and sets the corresponding bit in the
/// result (all other bits of the result are zeroed).
///
/// # Intrinsic (when available BMI1)
///
/// [`BLSI`](http://www.felixcloutier.com/x86/BLSI.html): Extract lowest set
/// isolated bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blsi<T: BLSI>(x: T) -> T {
    BLSI::blsi(x)
}
