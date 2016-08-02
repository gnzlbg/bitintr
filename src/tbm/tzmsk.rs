use int::Int;
use alg;

pub trait TZMSK {
    fn tzmsk(self) -> Self;
}

impl<T: Int> TZMSK for T {
    fn tzmsk(self) -> T {
        alg::tbm::tzmsk(self)
    }
}

/// Sets all bits below the least significant one of `x` and clears all other
/// bits.
///
/// If the least significant bit of `x` is 1, it returns zero.
///
/// # Intrinsic (when available TBM)
///
/// `TZMSK`: Mask from trailing zeros.
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn tzmsk<T: TZMSK>(x: T) -> T {
    TZMSK::tzmsk(x)
}
