use std::mem;

pub trait MULX {
    fn mulx(self, Self, &mut Self) -> Self;
}

impl MULX for u8 {
    fn mulx(self, y: u8, hi: &mut u8) -> u8 {
        let result: u16 = (self as u16) * (y as u16);
        *hi = (result >> 8) as u8;
        result as u8
    }
}

impl MULX for u16 {
    fn mulx(self, y: u16, hi: &mut u16) -> u16 {
        let result: u32 = (self as u32) * (y as u32);
        *hi = (result >> 16) as u16;
        result as u16
    }
}

impl MULX for u32 {
    fn mulx(self, y: u32, hi: &mut u32) -> u32 {
        let result: u64 = (self as u64) * (y as u64);
        *hi = (result >> 32) as u32;
        result as u32
    }
}

impl MULX for u64 {
    fn mulx(self, y: u64, hi: &mut u64) -> u64 {
        let result: u128 = (self as u128) * (y as u128);
        *hi = (result >> 64) as u64;
        result as u64
    }
}

impl MULX for usize {
    fn mulx(self, y: usize, hi: &mut usize) -> usize {
        match mem::size_of::<usize>() * 8 {
            8 => (self as u8).mulx(y as u8, unsafe { mem::transmute(hi) }) as usize,
            16 => (self as u16).mulx(y as u16, unsafe { mem::transmute(hi) }) as usize,
            32 => (self as u32).mulx(y as u32, unsafe { mem::transmute(hi) }) as usize,
            64 => (self as u64).mulx(y as u64, unsafe { mem::transmute(hi) }) as usize,
            _ => unreachable!(),
        }
    }
}

impl MULX for i8 {
    fn mulx(self, y: i8, hi: &mut i8) -> i8 {
        (self as u8).mulx(y as u8, unsafe { mem::transmute(hi) }) as i8
    }
}

impl MULX for i16 {
    fn mulx(self, y: i16, hi: &mut i16) -> i16 {
        (self as u16).mulx(y as u16, unsafe { mem::transmute(hi) }) as i16
    }
}

impl MULX for i32 {
    fn mulx(self, y: i32, hi: &mut i32) -> i32 {
        (self as u32).mulx(y as u32, unsafe { mem::transmute(hi) }) as i32
    }
}

impl MULX for i64 {
    fn mulx(self, y: i64, hi: &mut i64) -> i64 {
        (self as u64).mulx(y as u64, unsafe { mem::transmute(hi) }) as i64
    }
}


impl MULX for isize {
    fn mulx(self, y: isize, hi: &mut isize) -> isize {
        (self as usize).mulx(y as usize, unsafe { mem::transmute(hi) }) as isize
    }
}

/// Unsigned multiply without affecting flags.
///
/// Unsigned multiplication of `x` with `y` storing the high half of the result
/// in `hi` and returning the low half of the result.
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
///   let mut hi: u8 = 0;
///   let lo: u8 = mulx(a, b, &mut hi);
///   // result = 16384 = 0b0100_0000_0000_0000u16
///   //                    ^~hi~~~~~ ^~lo~~~~~
///   assert_eq!(lo, 0b0000_0000);
///   assert_eq!(hi, 0b0100_0000);
/// }
/// { // 16-bit
///   let a: u16 = 65_500;
///   let b: u16 = 65_500;
///   let mut hi: u16 = 0;
///   let lo: u16 = mulx(a, b, &mut hi);
///   // result = 4290250000 = 0b1111_1111_1011_1000_0000_0101_0001_0000u32
///   //                         ^~hi~~~~~~~~~~~~~~~ ^~lo~~~~~~~~~~~~~~~
///   assert_eq!(lo, 0b0000_0101_0001_0000);
///   assert_eq!(hi, 0b1111_1111_1011_1000);
/// }
/// { // 32-bit
///   let a: u32 = 4_294_967_200;
///   let b: u32 = 2;
///   let mut hi: u32 = 0;
///   let lo: u32 = mulx(a, b, &mut hi);
///   // result = 8589934400
///   //        = 0b0001_1111_1111_1111_1111_1111_1111_0100_0000u64
///   //            ^~hi ^~lo~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///   assert_eq!(lo, 0b1111_1111_1111_1111_1111_1111_0100_0000u32);
///   assert_eq!(hi, 0b0001u32);
/// }
/// { // 64-bit
///   let a: u64 = 9_223_372_036_854_775_800;
///   let b: u64 = 100;
///   let mut hi: u64 = 0;
///   let lo: u64 = mulx(a, b, &mut hi);
///   // result = 922337203685477580000
///   //        = 0b00110001_11111111_11111111_11111111_11111111_11111111_11111111_11111100_11100000u128
///   //            ^~hi~~~~ ^~lo~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///   assert_eq!(lo, 0b11111111_11111111_11111111_11111111_11111111_11111111_11111100_11100000u64);
///   assert_eq!(hi, 0b00110001u64);
/// }
/// ```
pub fn mulx<T: MULX>(x: T, y: T, hi: &mut T) -> T {
    x.mulx(y, hi)
}
