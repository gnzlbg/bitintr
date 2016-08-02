use int::Int;
use alg;

pub trait BLCS {
    fn blcs(self) -> Self;
}

impl<T: Int> BLCS for T {
    fn blcs(self) -> T {
        alg::tbm::blcs(self)
    }
}

/// Sets the least significant bit of `x`.
///
/// If there is no zero bit in `x`, it returns `x`.
///
/// # Intrinsic (when available TBM)
///
/// `BLCS`: Set lowest clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blcs<T: BLCS>(x: T) -> T {
    BLCS::blcs(x)
}
