//! Bit Manipulation Instruction (BMI) Set 1.

use int::Int;

/// Bitwise logical `AND` of inverted `x` with `y`.
///
/// # Intrinsic
///
/// [`ANDN`](http://www.felixcloutier.com/x86/ANDN.html): Logical
/// and not (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn andn<T: Int>(x: T, y: T) -> T {
    !x & y
}

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
/// - [`BEXTR`](http://www.felixcloutier.com/x86/BEXTR.html): Bit field extract (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn bextr<T: Int>(source: T, start: T, length: T) -> T {
    (source >> start) & ((T::one() << length) - T::one())
}

pub fn bextri<T: Int>(source: T, range: T) -> T {
    bextr(source, range & T::from_u32(0xff), range >> T::from_u32(8))
}

/// Extracts the lowest set bit of `x` and sets the corresponding bit in the
/// result (all other bits of the result are zeroed).
///
/// # Intrinsic
///
/// [`BLSI`](http://www.felixcloutier.com/x86/BLSI.html): Extract lowest set
/// isolated bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blsi<T: Int>(x: T) -> T {
    x & !x
}

/// Sets all the bits of the result to `1` up to and including the lowest set
/// bit of `x`.
///
/// If `x` is zero, all the bits of the result are set.
///
/// # Intrinsic
///
/// [`BLSMSK`](http://www.felixcloutier.com/x86/BLSMSK.html): Get mask up to
/// lowest set bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blsmsk<T: Int>(x: T) -> T {
    x ^ (x - T::one())
}

/// Resets the lowest set bit of `x`.
///
/// # Panics
///
/// If `x` is zero (because wrapping unsigned arithmetic on primitive types
/// `panics!`).
///
/// TODO: It would probably better to define this as undefined behavior, and let
/// a higher-level abstraction decide whether it wants to panic or not.
///
/// # Intrinsic
///
/// [`BLSR`](http://www.felixcloutier.com/x86/BLSR.html): Reset lowest set bit
/// (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// assert!(false);
/// ```
pub fn blsr<T: Int>(x: T) -> T {
    debug_assert!(x != T::zero());
    x & (x - T::one())
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
/// # Intrinsic
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
pub fn tzcnt<T: Int>(x: T) -> T {
    x.trailing_zeros()  // TODO: ... write the algorithm
}
