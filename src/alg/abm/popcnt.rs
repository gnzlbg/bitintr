use int::Int;

/// Counts the bits that are set.
///
/// **Keywords**: Population count, count ones, Hamming weight, Sideways sum.
///
/// # Intrinsic (when available: ABM / SSE4.2)
///
/// [`POPCNT`](http://www.felixcloutier.com/x86/POPCNT.html): Population Count (supports 16/32/64 bit registers).
///
/// Note: Intel considers it part of SSE4.2 but advertises it with its own CPUID
/// flag.
///
/// # Example
/// ```
/// use bitintr::abm::popcnt;
/// assert_eq!(popcnt(0b0101_1010u16), 4);
/// ```
pub fn popcnt<T: Int>(x: T) -> T {
    x.count_ones() // TODO: software emulation
}

pub trait POPCNT {
    fn popcnt(self) -> Self;
}

impl<T: Int> POPCNT for T {
    fn popcnt(self) -> T {
        popcnt(self)
    }
}
