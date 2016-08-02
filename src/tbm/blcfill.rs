use int::Int;
use alg;


pub trait BLCFILL {
    fn blcfill(self) -> Self;
}

impl<T: Int> BLCFILL for T {
    fn blcfill(self) -> T {
        alg::tbm::blcfill(self)
    }
}

/// Clears all bits below the least significant zero bit of `x`.
///
/// If there is no zero bit in `x`, it returns zero.
///
/// # Intrinsic (when available TBM)
///
/// `BLCFILL`: Fill from lowest clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blcfill<T: BLCFILL>(x: T) -> T {
    BLCFILL::blcfill(x)
}
