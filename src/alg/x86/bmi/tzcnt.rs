use int::Int;

/// Counts the number of trailing least significant zero bits.
///
/// When the source operand is 0, it returns its size in bits.
///
/// This is equivalent to searching for the least significant set bit and
/// returning its index.
///
/// **Keywords**: Count trailing zeros, Bit scan forward, find first set.
///
/// # Assembly Instructions
///
/// - [`TZCNT`](http://www.felixcloutier.com/x86/TZCNT.html):
///   - Description: Count the number of trailing zero bits.
///   - Architecture: x86.
///   - Instruction set: BMI.
///   - Registers: 16/32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::bmi::*;
///
/// assert_eq!(tzcnt(0b1001_0000u16), 4u16);
/// assert_eq!(0b0000_0000u32.tzcnt(), 32u32);
/// assert_eq!(tzcnt(0b0000_0001u64), 0u64);
/// ```
#[inline] pub fn tzcnt<T: Int>(x: T) -> T {
    x.trailing_zeros() // TODO: ... write the algorithm
}

/// Method version of [`tzcnt`](fn.tzcnt.html).
pub trait TZCNT {
    #[inline] fn tzcnt(self) -> Self;
}

impl<T: Int> TZCNT for T {
    #[inline] fn tzcnt(self) -> Self {
        tzcnt(self)
    }
}
