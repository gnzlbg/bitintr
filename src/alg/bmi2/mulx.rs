use std::mem;
use std::marker;

pub trait MULX {
    fn mulx(self, Self) -> (Self, Self) where Self: marker::Sized;
}

impl MULX for u8 {
    fn mulx(self, y: u8) -> (u8, u8) {
        let result: u16 = (self as u16) * (y as u16);
        let hi = (result >> 8) as u8;
        (result as u8, hi)
    }
}

impl MULX for u16 {
    fn mulx(self, y: u16) -> (u16, u16) {
        let result: u32 = (self as u32) * (y as u32);
        let hi = (result >> 16) as u16;
        (result as u16, hi)
    }
}

impl MULX for u32 {
    fn mulx(self, y: u32) -> (u32, u32) {
        let result: u64 = (self as u64) * (y as u64);
        let hi = (result >> 32) as u32;
        (result as u32, hi)
    }
}

#[cfg(RUSTC_IS_NIGHTLY)]
impl MULX for u64 {
    fn mulx(self, y: u64) -> (u64, u64) {
        let result: u128 = (self as u128) * (y as u128);
        let hi = (result >> 64) as u64;
        (result as u64, hi)
    }
}

#[cfg(not(RUSTC_IS_NIGHTLY))]
impl MULX for u64 {
    fn mulx(self, y: u64) -> (u64, u64) {
        let u1 = self & 0xffffffff;
        let v1 = y & 0xffffffff;
        let t = u1 * v1;
        let w3 = t & 0xffffffff;
        let k = t >> 32;

        let x = self >> 32;
        let t1 = (x * v1) + k;
        let k1 = t1 & 0xffffffff;
        let w1 = t1 >> 32;

        let y1 = y >> 32;
        let t2 = (u1 * y1) + k1;
        let k2 = t2 >> 32;

        let hi = (x * y1) + w1 + k2;
        let lo = (t2 << 32) + w3;
        (lo, hi)
    }
}

impl MULX for usize {
    fn mulx(self, y: usize) -> (usize, usize) {
        match mem::size_of::<usize>() * 8 {
            8 => {
                let tmp = (self as u8).mulx(y as u8);
                (tmp.0 as usize, tmp.1 as usize)
            }
            16 => {
                let tmp = (self as u16).mulx(y as u16);
                (tmp.0 as usize, tmp.1 as usize)
            }
            32 => {
                let tmp = (self as u32).mulx(y as u32);
                (tmp.0 as usize, tmp.1 as usize)
            }
            64 => {
                let tmp = (self as u64).mulx(y as u64);
                (tmp.0 as usize, tmp.1 as usize)
            }
            _ => unreachable!(),
        }
    }
}

impl MULX for i8 {
    fn mulx(self, y: i8) -> (i8, i8) {
        unsafe { mem::transmute((self as u8).mulx(y as u8)) }
    }
}

impl MULX for i16 {
    fn mulx(self, y: i16) -> (i16, i16) {
        unsafe { mem::transmute((self as u16).mulx(y as u16)) }
    }
}

impl MULX for i32 {
    fn mulx(self, y: i32) -> (i32, i32) {
        unsafe { mem::transmute((self as u32).mulx(y as u32)) }
    }
}

impl MULX for i64 {
    fn mulx(self, y: i64) -> (i64, i64) {
        unsafe { mem::transmute((self as u64).mulx(y as u64)) }
    }
}

impl MULX for isize {
    fn mulx(self, y: isize) -> (isize, isize) {
        unsafe { mem::transmute((self as usize).mulx(y as usize)) }
    }
}

/// Unsigned multiply without affecting flags.
///
/// Unsigned multiplication of `x` with `y` returning a pair `(lo, hi)` with
/// the low half and the high half of the result.
///
/// # Intrinsics (when available BMI2)
///
/// [`MULX`](http://www.felixcloutier.com/x86/MULX.html): Unsigned Multiply
/// Without Affecting Flags (supports 32/64 bit registers).
///
/// # Examples
///
/// ```
/// use bitintr::bmi2::mulx;
///
/// { // 8-bit
///   let a: u8 = 128;
///   let b: u8 = 128;
///   let (lo, hi): (u8, u8) = mulx(a, b);
///   // result = 16384 = 0b0100_0000_0000_0000u16
///   //                    ^~hi~~~~~ ^~lo~~~~~
///   assert_eq!(lo, 0b0000_0000);
///   assert_eq!(hi, 0b0100_0000);
/// }
/// { // 16-bit
///   let a: u16 = 65_500;
///   let b: u16 = 65_500;
///   let (lo, hi): (u16, u16)  = mulx(a, b);
///   // result = 4290250000 = 0b1111_1111_1011_1000_0000_0101_0001_0000u32
///   //                         ^~hi~~~~~~~~~~~~~~~ ^~lo~~~~~~~~~~~~~~~
///   assert_eq!(lo, 0b0000_0101_0001_0000);
///   assert_eq!(hi, 0b1111_1111_1011_1000);
/// }
/// { // 32-bit
///   let a: u32 = 4_294_967_200;
///   let b: u32 = 2;
///   let (lo, hi): (u32, u32)  = mulx(a, b);
///   // result = 8589934400
///   //        = 0b0001_1111_1111_1111_1111_1111_1111_0100_0000u64
///   //            ^~hi ^~lo~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///   assert_eq!(lo, 0b1111_1111_1111_1111_1111_1111_0100_0000u32);
///   assert_eq!(hi, 0b0001u32);
/// }
/// { // 64-bit
///   let a: u64 = 9_223_372_036_854_775_800;
///   let b: u64 = 100;
///   let (lo, hi): (u64, u64)  = mulx(a, b);
///   // result = 922337203685477580000
///   //        = 0b00110001_11111111_11111111_11111111_11111111_11111111_11111111_11111100_11100000u128
///   //            ^~hi~~~~ ^~lo~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///   assert_eq!(lo, 0b11111111_11111111_11111111_11111111_11111111_11111111_11111100_11100000u64);
///   assert_eq!(hi, 0b00110001u64);
/// }
/// { // 8-bit
///   let a: i8 = 128;
///   let b: i8 = 128;
///   let (lo, hi): (i8, i8) = mulx(a, b);
///   // result = _____ = 0b0100_0000_0000_0000u16
///   //                    ^~hi~~~~~ ^~lo~~~~~
///   assert_eq!(lo, 0b0000_0000);
///   assert_eq!(hi, 0b0100_0000);
/// }
/// { // 16-bit
///   let a: i16 = 65_500;
///   let b: i16 = 65_500;
///   let (lo, hi): (i16, i16)  = mulx(a, b);
///   // result = 4290250000 = 0b1111_1111_1011_1000_0000_0101_0001_0000u32
///   //                         ^~hi~~~~~~~~~~~~~~~ ^~lo~~~~~~~~~~~~~~~
///   assert_eq!(lo, 0b0000_0101_0001_0000);
///   assert_eq!(hi, 0b1111_1111_1011_1000);
/// }
/// ```
pub fn mulx<T: MULX>(x: T, y: T) -> (T, T) {
    x.mulx(y)
}
