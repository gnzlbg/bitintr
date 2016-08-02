use int::Int;
use alg;

pub trait BLCIC {
    fn blcic(self) -> Self;
}

impl<T: Int> BLCIC for T {
    fn blcic(self) -> T {
        alg::tbm::blcic(self)
    }
}

/// Sets the least significant bit of `x` and clears all other bits.
///
/// If there is no zero bit in `x`, it returns zero.
///
/// # Intrinsic (when available TBM)
///
/// `BLCIC`: Isolate lowest clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blcic<T: BLCIC>(x: T) -> T {
    BLCIC::blcic(x)
}
