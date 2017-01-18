use int::Int;

/// Zero the high bits of `x` at position >= `bit_position`.
///
/// # Panics
///
/// If `bit_position >= bit_size()` the behavior is undefined (panics in debug
/// builds).
///
/// # Intrinsics (when available BMI2)
///
/// [`BZHI`](http://www.felixcloutier.com/x86/BZHI.html): Zero high bits
/// starting with specified bit position (supports 32/64 bit registers).
///
/// # Examples
///
/// ```
/// use bitintr::bmi2::bzhi;
///
/// let n = 0b1111_0010u32;
/// let s = 0b0001_0010u32;
/// assert_eq!(bzhi(n, 5), s);
/// ```
pub fn bzhi<T: Int>(x: T, bit_position: T) -> T {
    debug_assert!(bit_position < T::bit_size());
    x & ((T::one() << bit_position) - T::one())
}

pub trait BZHI {
    fn bzhi(self, Self) -> Self;
}

impl<T: Int> BZHI for T {
    fn bzhi(self, y: Self) -> Self {
        bzhi(self, y)
    }
}
