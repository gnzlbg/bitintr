use int::Int;
use alg;

pub trait BLSFILL {
    fn blsfill(self) -> Self;
}

impl<T: Int> BLSFILL for T {
    fn blsfill(self) -> T {
        alg::tbm::blsfill(self)
    }
}

/// Sets all bits of `x` below the least significant one.
///
/// If there is no set bit in `x`, it sets all the bits.
///
/// # Intrinsic (when available TBM)
///
/// `BLSFILL`: Fill from lowest set bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blsfill<T: BLSFILL>(x: T) -> T {
    BLSFILL::blsfill(x)
}
