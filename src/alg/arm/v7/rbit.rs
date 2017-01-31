use int::Int;
use std;

#[cfg(RUSTC_IS_NIGHTLY)]
#[allow(dead_code)]
extern "C" {
    #[link_name="llvm.bitreverse.i8"]
    fn bitrev_i8(i: i8) -> i8;
    #[link_name="llvm.bitreverse.i16"]
    fn bitrev_i16(i: i16) -> i16;
    #[link_name="llvm.bitreverse.i32"]
    fn bitrev_i32(i: i32) -> i32;
    #[link_name="llvm.bitreverse.i64"]
    fn bitrev_i64(i: i64) -> i64;
}

// generic bitreverse for unsigned integers
#[allow(dead_code)]
#[inline]
fn bitreverse_unsigned<T: Int>(y: T) -> T {
    let mut x = y;
    let width = T::byte_size();
    let k = T::bit_size() - T::from_u32(1);

    {
        let mut up0 = |i, l, r| if k & i > T::from_u32(0) {
            x = ((x & l).wrapping_shl(i)) | ((x & r).wrapping_shr(i));
        };

        up0(T::from_u32(1),
            T::from_u64(0x5555555555555555u64),
            T::from_u64(0xAAAAAAAAAAAAAAAAu64));
        up0(T::from_u32(2),
            T::from_u64(0x3333333333333333u64),
            T::from_u64(0xCCCCCCCCCCCCCCCCu64));
        up0(T::from_u32(4),
            T::from_u64(0x0F0F0F0F0F0F0F0Fu64),
            T::from_u64(0xF0F0F0F0F0F0F0F0u64));
    }
    {
        let mut up1 = |i, s, l, r| if width > i && (k & s > T::from_u32(0)) {
            x = ((x & l).wrapping_shl(s)) | ((x & r).wrapping_shr(s));
        };

        up1(T::from_u32(1),
            T::from_u32(8),
            T::from_u64(0x00FF00FF00FF00FFu64),
            T::from_u64(0xFF00FF00FF00FF00u64));
        up1(T::from_u32(2),
            T::from_u32(16),
            T::from_u64(0x0000FFFF0000FFFFu64),
            T::from_u64(0xFFFF0000FFFF0000u64));
        up1(T::from_u32(4),
            T::from_u32(32),
            T::from_u64(0x00000000FFFFFFFFu64),
            T::from_u64(0xFFFFFFFF00000000u64));
    }
    x
}

// generic bitreverse:
#[allow(dead_code)]
#[inline]
fn bitreverse<T: Int>(y: T) -> T {
    T::from_unsigned(bitreverse_unsigned(y.to_unsigned()))
}

#[allow(dead_code)]
#[inline]
fn bitreverse_u8(i: u8) -> u8 {
    (((i as u64).wrapping_mul(0x80200802u64) & 0x0884422110u64)
        .wrapping_mul(0x0101010101u64)
        .wrapping_shr(32)) as u8
}

// bitreverse for i8 using 4 u64 multipy
#[inline]
fn bitreverse_i8(i: i8) -> i8 {
    #[cfg(RUSTC_IS_NIGHTLY)]
    {
        // BUG: https://github.com/gnzlbg/bitintr/issues/4
        // TL;DR: llvm.bitreverse.i8 generates very bad code in LLVM 3.9
        //
        // unsafe { bitrev_i8(i) }
        bitreverse_u8(i as u8) as i8
    }
    #[cfg(not(RUSTC_IS_NIGHTLY))]
    {
        bitreverse_u8(i as u8) as i8
    }
}

#[inline]
fn bitreverse_i16(i: i16) -> i16 {
    #[cfg(RUSTC_IS_NIGHTLY)]
    {
        unsafe { bitrev_i16(i) }
    }
    #[cfg(not(RUSTC_IS_NIGHTLY))]
    {
        bitreverse(i)
    }
}

#[inline]
fn bitreverse_i32(i: i32) -> i32 {
    #[cfg(RUSTC_IS_NIGHTLY)]
    {
        unsafe { bitrev_i32(i) }
    }
    #[cfg(not(RUSTC_IS_NIGHTLY))]
    {
        bitreverse(i)
    }
}

#[inline]
fn bitreverse_i64(i: i64) -> i64 {
    #[cfg(RUSTC_IS_NIGHTLY)]
    {
        unsafe { bitrev_i64(i) }
    }
    #[cfg(not(RUSTC_IS_NIGHTLY))]
    {
        bitreverse(i)
    }
}

/// Method version of [`rbit`](fn.rbit.html).
pub trait RBit: Sized {
    #[inline]
    fn rbit(self) -> Self;
}

impl RBit for i8 {
    #[inline]
    fn rbit(self) -> i8 {
        bitreverse_i8(self)
    }
}

impl RBit for i16 {
    #[inline]
    fn rbit(self) -> i16 {
        bitreverse_i16(self)
    }
}

impl RBit for i32 {
    #[inline]
    fn rbit(self) -> i32 {
        bitreverse_i32(self)
    }
}

impl RBit for i64 {
    #[inline]
    fn rbit(self) -> i64 {
        bitreverse_i64(self)
    }
}

impl RBit for isize {
    #[inline]
    fn rbit(self) -> isize {
        match std::mem::size_of::<isize>() {
            32 => bitreverse_i32(self as i32) as isize,
            64 => bitreverse_i64(self as i64) as isize,
            _ => unreachable!(),
        }
    }
}

impl RBit for u8 {
    #[inline]
    fn rbit(self) -> u8 {
        (self as i8).rbit() as u8
    }
}

impl RBit for u16 {
    #[inline]
    fn rbit(self) -> u16 {
        (self as i16).rbit() as u16

    }
}

impl RBit for u32 {
    #[inline]
    fn rbit(self) -> u32 {
        (self as i32).rbit() as u32
    }
}

impl RBit for u64 {
    #[inline]
    fn rbit(self) -> u64 {
        (self as i64).rbit() as u64
    }
}

impl RBit for usize {
    #[inline]
    fn rbit(self) -> usize {
        (self as isize).rbit() as usize
    }
}

/// Reverse the bit order.
///
/// # Assembly Instructions
///
/// - [`RBIT`](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0487a.k_10775/index.html):
///   - Description: Reverse Bits reverses the bit order in a register.
///   - Architecture: ARMv7, ARMv8.
///   - Registers: 32 (v7) / 64 (v8) bits.
///
/// # Example
///
/// ```
/// use bitintr::arm::v7::*;
///
/// assert_eq!(rbit(0b0011_0001u8), 0b1000_1100u8);
//  assert_eq!(rbit(0b0000_0000_0100_1011u16), 0b1101_0010_0000_0000u16);
/// assert_eq!(rbit(0b11111111u32), 0b11111111_00000000_00000000_00000000u32);
/// ```
#[inline]
pub fn rbit<T: RBit>(y: T) -> T {
    y.rbit()
}

#[cfg(test)]
mod tests {
    use arm;
    use int::Int;
    use std::fmt::Debug;
    use alg::arm::v7::RBit;
    #[cfg(RUSTC_IS_NIGHTLY)]
    use super::bitreverse;

    fn rbit_invariant<T: RBit + Int + Debug>(i: T) {
        let j = arm::v7::rbit(i);
        assert_eq!(i, arm::v7::rbit(j));
    }

    #[test]
    fn rbit_u8() {
        (0..u8::max_value())
            .map(|x| {
                rbit_invariant(x);
                rbit_invariant(x as i8)
            })
            .count();
        #[cfg(RUSTC_IS_NIGHTLY)]
        {
            (0..u8::max_value())
                .map(|x| {
                    assert_eq!(arm::v7::rbit(x), bitreverse(x));
                    x
                })
                .count();
        }
    }
    #[test]
    fn rbit_u16() {
        (0..u16::max_value())
            .map(|x| {
                rbit_invariant(x);
                rbit_invariant(x as i16)
            })
            .count();
        #[cfg(RUSTC_IS_NIGHTLY)]
        {
            (0..u16::max_value())
                .map(|x| {
                    assert_eq!(arm::v7::rbit(x), bitreverse(x));
                    x
                })
                .count();
        }
    }
    #[test]
    fn rbit_u32() {
        (0..u32::max_value())
            .take(1000000)
            .map(|x| {
                rbit_invariant(x);
                rbit_invariant(x as i32)
            })
            .count();
        #[cfg(RUSTC_IS_NIGHTLY)]
        {
            (0..u32::max_value())
                .take(1000000)
                .map(|x| {
                    assert_eq!(arm::v7::rbit(x), bitreverse(x));
                    x
                })
                .count();
        }
    }
    #[test]
    fn rbit_u64() {
        (0..u64::max_value())
            .take(1000000)
            .map(|x| {
                rbit_invariant(x);
                rbit_invariant(x as i64)
            })
            .count();
        #[cfg(RUSTC_IS_NIGHTLY)]
        {
            (0..u64::max_value())
                .take(1000000)
                .map(|x| {
                    assert_eq!(arm::v7::rbit(x), bitreverse(x));
                    x
                })
                .count();
        }
    }

    #[test]
    fn rbit_tests() {
        use arm::v7::rbit;

        {
            // width: 8
            let o_u8 = 0b1101_0011u8;
            let r_u8 = 0b1100_1011u8;
            assert_eq!(o_u8, 211);
            assert_eq!(r_u8, 203);
            assert_eq!(rbit(o_u8), r_u8);
            assert_eq!(rbit(o_u8 as i8), r_u8 as i8);
        }
        {
            //width: 16
            let o_u16 = 0b1101_0011_1110_1010u16;
            let r_u16 = 0b0101_0111_1100_1011u16;
            assert_eq!(rbit(o_u16), r_u16);
            assert_eq!(rbit(o_u16 as i16), r_u16 as i16);
        }
        {
            // width: 32
            let o_u32 = 0b1101_0011_1110_1010_1101_0011_1010_1010u32;
            let r_u32 = 0b0101_0101_1100_1011_0101_0111_1100_1011u32;
            assert_eq!(rbit(o_u32), r_u32);
            assert_eq!(rbit(o_u32 as i32), r_u32 as i32);
        }
        {
            // width: 64
            let o_u64 = 0b1101_0011_0010_1010_1111_0011_1010_1010_1101_0011_1110_1010_1101_0011_1010_1010u64;
            let r_u64 = 0b0101_0101_1100_1011_0101_0111_1100_1011_0101_0101_1100_1111_0101_0100_1100_1011u64;
            assert_eq!(rbit(o_u64), r_u64);
            assert_eq!(rbit(o_u64 as i64), r_u64 as i64);
        }

    }
}
