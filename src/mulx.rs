//! mulx

/// Unsigned multiply without affecting flags.
pub trait Mulx: crate::marker::Sized {
    /// Unsigned multiply without affecting flags.
    ///
    /// Unsigned multiplication of `x` with `y` returning a pair `(lo, hi)`
    /// with the low half and the high half of the result.
    ///
    /// # Instructions
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
    /// # use bitintr::*;
    /// { // 8-bit
    ///   let a: u8 = 128;
    ///   let b: u8 = 128;
    ///   let (lo, hi): (u8, u8) = a.mulx(b);
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
    ///   let (lo, hi): (u32, u32)  = a.mulx(b);
    ///   // result = 8589934400
    ///   //        = 0b0001_1111_1111_1111_1111_1111_1111_0100_0000u64
    ///   //            ^~hi ^~lo~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///   assert_eq!(lo, 0b1111_1111_1111_1111_1111_1111_0100_0000u32);
    ///   assert_eq!(hi, 0b0001u32);
    /// }
    /// { // 64-bit
    ///   let a: u64 = 9_223_372_036_854_775_800;
    ///   let b: u64 = 100;
    ///   let (lo, hi): (u64, u64)  = a.mulx(b);
    ///   // result = 922337203685477580000
    ///   //        = 0b00110001_11111111_11111111_11111111_11111111_11111111_11111111_11111100_11100000u128
    ///   //            ^~hi~~~~ ^~lo~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///   assert_eq!(lo, 0b11111111_11111111_11111111_11111111_11111111_11111111_11111100_11100000u64);
    ///   assert_eq!(hi, 0b00110001u64);
    /// }
    /// { // 8-bit
    ///   let a: i8 = 128u8 as i8;
    ///   let b: i8 = 128u8 as i8;
    ///   let (lo, hi): (i8, i8) = a.mulx(b);
    ///   // result = _____ = 0b0100_0000_0000_0000u16
    ///   //                    ^~hi~~~~~ ^~lo~~~~~
    ///   assert_eq!(lo, 0b0000_0000);
    ///   assert_eq!(hi, 0b0100_0000);
    /// }
    /// { // 16-bit
    ///   let a: i16 = 65_500u16 as i16;
    ///   let b: i16 = 65_500u16 as i16;
    ///   let (lo, hi): (i16, i16)  = a.mulx(b);
    ///   // result = 4290250000 = 0b1111_1111_1011_1000_0000_0101_0001_0000u32
    ///   //                         ^~hi~~~~~~~~~~~~~~~ ^~lo~~~~~~~~~~~~~~~
    ///   assert_eq!(lo, 0b0000_0101_0001_0000u16 as i16);
    ///   assert_eq!(hi, 0b1111_1111_1011_1000u16 as i16);
    /// }
    /// ```
    fn mulx(self, y: Self) -> (Self, Self);
}

macro_rules! impl_umulx {
    ($id:ident, $id_l:ident) => {
        #[allow(clippy::use_self)]
        impl Mulx for $id {
            #[inline]
            fn mulx(self, y: Self) -> (Self, Self) {
                const BIT_WIDTH: $id_l =
                    (crate::mem::size_of::<$id>() * 8) as $id_l;
                let x = self;
                let result: $id_l = (x as $id_l) * (y as $id_l);
                let hi = (result >> BIT_WIDTH) as Self;
                (result as Self, hi)
            }
        }
    };
}

impl_umulx!(u8, u16);
impl_umulx!(u16, u32);
impl_umulx!(u32, u64);
impl_umulx!(u64, u128);

macro_rules! impl_smulx {
    ($id:ident, $uid:ident) => {
        impl Mulx for $id {
            #[inline]
            fn mulx(self, y: Self) -> (Self, Self) {
                let ux = self as $uid;
                let uy = y as $uid;
                let (rx, ry) = ux.mulx(uy);
                (rx as _, ry as _)
            }
        }
    };
}

impl_smulx!(i8, u8);
impl_smulx!(i16, u16);
impl_smulx!(i32, u32);
impl_smulx!(i64, u64);
