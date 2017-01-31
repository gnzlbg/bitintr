use int::Int;

#[inline]
fn mulx_u8(x: u8, y: u8) -> (u8, u8) {
    let result: u16 = (x as u16) * (y as u16);
    let hi = (result >> 8) as u8;
    (result as u8, hi)
}


#[inline]
fn mulx_u16(x: u16, y: u16) -> (u16, u16) {
    let result: u32 = (x as u32) * (y as u32);
    let hi = (result >> 16) as u16;
    (result as u16, hi)
}

#[inline]
fn mulx_u32(x: u32, y: u32) -> (u32, u32) {
    let result: u64 = (x as u64) * (y as u64);
    let hi = (result >> 32) as u32;
    (result as u32, hi)
}

#[inline]
fn mulx_u64(x: u64, y: u64) -> (u64, u64) {
    #[cfg(RUSTC_IS_NIGHTLY)]
    {
        let result: u128 = (x as u128) * (y as u128);
        let hi = (result >> 64) as u64;
        (result as u64, hi)
    }
    #[cfg(not(RUSTC_IS_NIGHTLY))]
    {
        let u1 = x & 0xffffffff;
        let v1 = y & 0xffffffff;
        let t = u1 * v1;
        let w3 = t & 0xffffffff;
        let k = t >> 32;

        let x = x >> 32;
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

/// Unsigned multiply without affecting flags.
///
/// Unsigned multiplication of `x` with `y` returning a pair `(lo, hi)` with
/// the low half and the high half of the result.
///
/// # Assembly Instructions
///
/// - [`MULX`](http://www.felixcloutier.com/x86/MULX.html):
///   - Description: Unsigned multiply without affecting flags.
///   - Architecture: x86.
///   - Instruction set: BMI2.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::bmi2::*;
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
///   let (lo, hi): (u16, u16)  = a.mulx(b);
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
///   let a: i8 = 128u8 as i8;
///   let b: i8 = 128u8 as i8;
///   let (lo, hi): (i8, i8) = mulx(a, b);
///   // result = _____ = 0b0100_0000_0000_0000u16
///   //                    ^~hi~~~~~ ^~lo~~~~~
///   assert_eq!(lo, 0b0000_0000);
///   assert_eq!(hi, 0b0100_0000);
/// }
/// { // 16-bit
///   let a: i16 = 65_500u16 as i16;
///   let b: i16 = 65_500u16 as i16;
///   let (lo, hi): (i16, i16)  = mulx(a, b);
///   // result = 4290250000 = 0b1111_1111_1011_1000_0000_0101_0001_0000u32
///   //                         ^~hi~~~~~~~~~~~~~~~ ^~lo~~~~~~~~~~~~~~~
///   assert_eq!(lo, 0b0000_0101_0001_0000u16 as i16);
///   assert_eq!(hi, 0b1111_1111_1011_1000u16 as i16);
/// }
/// ```
#[inline]
pub fn mulx<T: Int>(x: T, y: T) -> (T, T) {

    match T::bit_size().to_u8() {
        8 => {
            let (x, y) = mulx_u8(x.to_u8(), y.to_u8());
            (T::from_u8(x), T::from_u8(y))
        }
        16 => {
            let (x, y) = mulx_u16(x.to_u16(), y.to_u16());
            (T::from_u16(x), T::from_u16(y))
        }
        32 => {
            let (x, y) = mulx_u32(x.to_u32(), y.to_u32());
            (T::from_u32(x), T::from_u32(y))
        }
        64 => {
            let (x, y) = mulx_u64(x.to_u64(), y.to_u64());
            (T::from_u64(x), T::from_u64(y))
        }
        _ => unreachable!(),
    }
}

/// Method version of [`mulx`](fn.mulx.html).
pub trait MULX: Sized {
    #[inline]
    fn mulx(self, Self) -> (Self, Self);
}

impl<T: Int> MULX for T {
    #[inline]
    fn mulx(self, x: Self) -> (Self, Self) {
        mulx(self, x)
    }
}
