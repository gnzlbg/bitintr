use int::Int;
use alg;

pub trait BLCMSK {
    fn blcmsk(self) -> Self;
}

impl<T: Int> BLCMSK for T {
    fn blcmsk(self) -> T {
        alg::tbm::blcmsk(self)
    }
}

/// Sets the least significant bit of `x` and clears all bits above that bit.
///
/// If there is no zero bit in `x`, it sets all the bits.
///
/// # Intrinsic (when available TBM)
///
/// `BLCMSK`: Mask from lowest clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blcmsk<T: BLCMSK>(x: T) -> T {
    BLCMSK::blcmsk(x)
}
