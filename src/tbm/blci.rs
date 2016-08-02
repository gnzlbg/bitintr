use int::Int;
use alg;

pub trait BLCI {
    fn blci(self) -> Self;
}

impl<T: Int> BLCI for T {
    fn blci(self) -> T {
        alg::tbm::blci(self)
    }
}

/// Sets all bits of `x` to 1 except for the least significant zero bit.
///
/// If there is no zero bit in `x`, it sets all bits.
///
/// # Intrinsic (when available TBM)
///
/// `BLCI`: Isolate lowest clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blci<T: BLCI>(x: T) -> T {
    BLCI::blci(x)
}
