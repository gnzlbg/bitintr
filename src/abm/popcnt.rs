use int::Int;

pub trait POPCNT {
    fn popcnt(self) -> Self;
}

impl<T: Int> POPCNT for T {
    fn popcnt(self) -> T {
        self.count_ones()
    }
}

/// Counts the bits that are set.
///
/// **Keywords**: Population count, count ones, Hamming weight, Sideways sum.
///
/// # Intrinsic (when available: ABM / SSE4.2)
///
/// [`POPCNT`](http://www.felixcloutier.com/x86/POPCNT.html): Population Count (supports 16/32/64 bit registers).
///
/// # Example
/// ```
/// assert!(false);
/// ```
pub fn popcnt<T: POPCNT>(x: T) -> T {
    POPCNT::popcnt(x)
}
