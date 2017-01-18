use int::Int;

/// Extract lowest set isolated bit.
///
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
/// use bitintr::bmi::blsi;
///
/// assert_eq!(blsi(0b1101_0000u8), 0b0001_0000u8);
/// assert_eq!(blsi(0b0100_1000u8), 0b0000_1000u8);
/// ```
pub fn blsi<T: Int>(x: T) -> T {
    x & x.wrapping_neg()
}

pub trait BLSI {
    fn blsi(self) -> Self;
}

impl<T: Int> BLSI for T {
    fn blsi(self) -> Self {
        blsi(self)
    }
}
