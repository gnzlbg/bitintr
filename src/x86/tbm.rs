//! Trailing Bit Manipulation (TBM) instruction set.
//!
//! For a quick overview see
//! [wikipedia](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#TBM_.28Trailing_Bit_Manipulation.29).
//! The reference is [AMD64 Architecture Programmer's Manual, Volume 3:
//! General-Purpose and System
//! Instructions](http://support.amd.com/TechDocs/24594.pdf).
//!
//! It consists of the following instructions:
//!
//! - `BEXTRi` (TODO).
//! - `BLCFILL`.
//! - `BLCI`.
//! - `BLCIC`.
//! - `BLCMSK`.
//! - `BLCS`.
//! - `BLSFILL`.
//! - `BLSIC`.
//! - `T1MSKC`.
//! - `TZMSK`.

use x86;
use alg;

use int::IntF32T64;

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
pub fn bextri<T: IntF32T64>(source: T, start: T, length: T) -> T {
    x86::bmi::bextr(source, start, length)
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
pub fn blcfill<T: IntF32T64>(x: T) -> T {
    alg::tbm::blcfill(x)
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
pub fn blci<T: IntF32T64>(x: T) -> T {
    alg::tbm::blci(x)
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
pub fn blcic<T: IntF32T64>(x: T) -> T {
    alg::tbm::blcic(x)
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
pub fn blcmsk<T: IntF32T64>(x: T) -> T {
    alg::tbm::blcmsk(x)
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
pub fn blcs<T: IntF32T64>(x: T) -> T {
    alg::tbm::blcs(x)
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
pub fn blsfill<T: IntF32T64>(x: T) -> T {
    alg::tbm::blsfill(x)
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
pub fn blsic<T: IntF32T64>(x: T) -> T {
    alg::tbm::blsic(x)
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
pub fn t1mskc<T: IntF32T64>(x: T) -> T {
    alg::tbm::t1mskc(x)
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
pub fn tzmsk<T: IntF32T64>(x: T) -> T {
    alg::tbm::tzmsk(x)
}
