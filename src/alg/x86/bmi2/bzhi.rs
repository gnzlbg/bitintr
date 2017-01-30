use int::Int;

/// Zero the high bits of `x` at position >= `bit_position`.
///
/// See [`x86::bmi2::bzhi`](fn.bzhi.html).
#[inline] pub fn bzhi<T: Int>(x: T, bit_position: T) -> T {
    debug_assert!(bit_position < T::bit_size());
    x & ((T::one() << bit_position) - T::one())
}

/// Method version of [`bzhi`](fn.bzhi.html).
pub trait BZHI {
    #[inline] fn bzhi(self, Self) -> Self;
}

impl<T: Int> BZHI for T {
    #[inline] fn bzhi(self, y: Self) -> Self {
        bzhi(self, y)
    }
}
