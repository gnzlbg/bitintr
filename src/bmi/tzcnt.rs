use int::Int;

pub trait TZCNT {
    fn tzcnt(self) -> Self;
}

impl<T: Int> TZCNT for T {
    fn tzcnt(self) -> Self {
        self.trailing_zeros()
    }
}

/// Counts the number of trailing least significant zero bits.
///
/// When the source operand is 0, it returns its size in bits.
///
/// This is equivalent to searching for the least significant set bit and
/// returning its index.
///
/// **Keywords**: Count trailing zeros, Bit scan forward, find first set.
///
/// # Intrinsic (when available BMI1)
///
/// [`TZCNT`](http://www.felixcloutier.com/x86/TZCNT.html): Count the number of
/// trailing zero bits (supports 16/32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::x86::bmi::tzcnt;
///
/// assert_eq!(tzcnt(0b1001_0000u16), 4u16);
/// assert_eq!(tzcnt(0b0000_0000u32), 32u32);
/// assert_eq!(tzcnt(0b0000_0001u64), 0u64);
/// ```
pub fn tzcnt<T: TZCNT>(x: T) -> T {
    TZCNT::tzcnt(x)
}
