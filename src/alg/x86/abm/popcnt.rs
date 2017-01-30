use int::Int;

/// Counts the bits that are set.
///
/// **Keywords**: Population count, count ones, Hamming weight, Sideways sum.
///
/// # Assembly Instructions
///
/// - [`POPCNT`](http://www.felixcloutier.com/x86/POPCNT.html):
///   - Description: Population Count.
///   - Architecture: x86.
///   - Instruction set: ABM, SSE 4.2.
///   - Registers: 16/32/64 bit.
///   - Note: Intel considers it part of SSE4.2 but advertises it with its own
///   CPUID flag.
///
/// # Example
/// ```
/// use bitintr::x86::abm::*;
/// assert_eq!(popcnt(0b0101_1010u16), 4);
/// assert_eq!(0b0101_1010u16.popcnt(), 4);
/// ```
#[inline] pub fn popcnt<T: Int>(x: T) -> T {
    x.count_ones() // TODO: software emulation
}

/// Method version of [`popcnt`](fn.popcnt.html).
pub trait POPCNT {
    #[inline] fn popcnt(self) -> Self;
}

impl<T: Int> POPCNT for T {
    #[inline] fn popcnt(self) -> T {
        popcnt(self)
    }
}
