use int::Int;
use alg;

pub trait T1MSKC {
    fn t1mskc(self) -> Self;
}

impl<T: Int> T1MSKC for T {
    fn t1mskc(self) -> T {
        alg::tbm::t1mskc(self)
    }
}

/// Clears all bits below the least significant zero of `x` and sets all other
/// bits.
///
/// If the least significant bit of `x` is 0, it sets all bits.
///
/// # Intrinsic (when available TBM)
/// `T1MSKC`: Inverse mask from trailing ones (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn t1mskc<T: T1MSKC>(x: T) -> T {
    T1MSKC::t1mskc(x)
}
