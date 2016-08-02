//! Trailing Bit Manipulation (TBM) instruction set.

use int::Int;

use alg::bmi;

/// Bit Field Extract (immediate form).
///
/// Extracts bits in range [`start`, `start` + `length`) from the `source` to
/// the least significant bits of the result.
///
/// Bits [7,0] of `range` specify the index to the first bit in the range to be
/// extracted, and bits [15,8] specify the length of the range.
///
/// Only bits up to `std::mem::size_of::<T>() - 1` are extracted.
///
/// The extracted bits are written in the result starting from the
/// least-significant bit. The high-order bits of the result are zeroed.
///
/// # Intrinsic
///
/// `BEXTRI`: Bit field extract (immediate, supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn bextri<T: Int>(source: T, start: T, length: T) -> T {
    bmi::bextr(source, start, length)
}

/// Clears all bits below the least significant zero bit of `x`.
///
/// If there is no zero bit in `x`, it returns zero.
///
/// # Intrinsic
///
/// `BLCFILL`: Fill from lowest clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blcfill<T: Int>(x: T) -> T {
    x & (x + T::one())
}

/// Sets all bits of `x` to 1 except for the least significant zero bit.
///
/// If there is no zero bit in `x`, it sets all bits.
///
/// # Intrinsic
///
/// `BLCI`: Isolate lowest clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blci<T: Int>(x: T) -> T {
    x | !(x + T::one())
}

/// Sets the least significant bit of `x` and clears all other bits.
///
/// If there is no zero bit in `x`, it returns zero.
///
/// # Intrinsic
///
/// `BLCIC`: Isolate lowest clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blcic<T: Int>(x: T) -> T {
    !x & (x + T::one())
}

/// Sets the least significant bit of `x` and clears all bits above that bit.
///
/// If there is no zero bit in `x`, it sets all the bits.
///
/// # Intrinsic
///
/// `BLCMSK`: Mask from lowest clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blcmsk<T: Int>(x: T) -> T {
    x ^ (x + T::one())
}

/// Sets the least significant bit of `x`.
///
/// If there is no zero bit in `x`, it returns `x`.
///
/// # Intrinsic
///
/// `BLCS`: Set lowest clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blcs<T: Int>(x: T) -> T {
    x | (x + T::one())
}

/// Sets all bits of `x` below the least significant one.
///
/// If there is no set bit in `x`, it sets all the bits.
///
/// #Intrinsic
///
/// `BLSFILL`: Fill from lowest set bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blsfill<T: Int>(x: T) -> T {
    x | (x - T::one())
}

/// Clears least significant bit and sets all other bits.
///
/// If there is no set bit in `x`, it sets all the bits.
///
/// #Intrinsic
///
/// `BLSIC`: Isolate lowest set bit and complement (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blsic<T: Int>(x: T) -> T {
    !x | (x - T::one())
}

/// Clears all bits below the least significant zero of `x` and sets all other
/// bits.
///
/// If the least significant bit of `x` is 0, it sets all bits.
///
/// #Intrinsic
/// `T1MSKC`: Inverse mask from trailing ones (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn t1mskc<T: Int>(x: T) -> T {
    !x | (x + T::one())
}

/// Sets all bits below the least significant one of `x` and clears all other
/// bits.
///
/// If the least significant bit of `x` is 1, it returns zero.
///
/// #Intrinsic
///
/// `TZMSK`: Mask from trailing zeros.
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn tzmsk<T: Int>(x: T) -> T {
    !x & (x - T::one())
}
