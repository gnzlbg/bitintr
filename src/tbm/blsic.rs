use int::Int;
use alg;

pub trait BLSIC {
    fn blsic(self) -> Self;
}

impl<T: Int> BLSIC for T {
    fn blsic(self) -> T {
        alg::tbm::blsic(self)
    }
}

/// Clears least significant bit and sets all other bits.
///
/// If there is no set bit in `x`, it sets all the bits.
///
/// # Intrinsic (when available TBM)
///
/// `BLSIC`: Isolate lowest set bit and complement (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blsic<T: BLSIC>(x: T) -> T {
    BLSIC::blsic(x)
}
