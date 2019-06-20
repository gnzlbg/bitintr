//! rbit

/// Bit reverse
pub trait Rbit {
    /// Reverse the bit order.
    ///
    /// # Instructions
    ///
    /// - [`RBIT`](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.
    ///   ddi0487a.k_10775/index.html):
    ///   - Description: Reverse Bits reverses the bit order in a register.
    ///   - Architecture: ARMv7, ARMv8.
    ///   - Registers: 32 (v7) / 64 (v8) bits.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0011_0001_u8.rbit(), 0b1000_1100_u8);
    /// assert_eq!(0b0000_0000_0100_1011_u16.rbit(), 0b1101_0010_0000_0000_u16);
    /// assert_eq!(
    ///     0b11111111_u32.rbit(),
    ///     0b11111111_00000000_00000000_00000000_u32
    /// );
    /// ```
    fn rbit(self) -> Self;
}

cfg_if! {
    // The reverse_bits methods have been stabilized and should land
    // in Rust 1.37.0 .
    if #[cfg(bitintr_nightly)] {
        macro_rules! impl_rbit {
            ($id:ident) => {
                impl Rbit for $id {
                    #[inline]
                    fn rbit(self) -> Self {
                        self.reverse_bits()
                    }
                }
            };
        }
    } else {
        macro_rules! impl_rbit {
            ($id:ident) => {
                impl Rbit for $id {
                    #[inline]
                    fn rbit(self) -> Self {
                        if crate::mem::size_of::<Self>() == 1 {
                            return (((self as u8 as u64).wrapping_mul(0x80200802_u64)
                              & 0x0884422110_u64)
                             .wrapping_mul(0x0101010101_u64)
                             .wrapping_shr(32)) as Self;
                        }
                        let mut x = self;
                        let byte_width = crate::mem::size_of::<Self>() as u32;
                        let bit_width = byte_width * 8;
                        let k = bit_width - 1;

                        {
                            let mut up0 = |i: u32, l: u64, r: u64| {
                                if k & i > 0 {
                                    x = (((x as u64 & l).wrapping_shl(i))
                                         | ((x as u64 & r).wrapping_shr(i)))
                                        as Self;
                                }
                            };

                            up0(
                                1,
                                0x5555555555555555_u64,
                                0xAAAAAAAAAAAAAAAA_u64,
                            );
                            up0(
                                2,
                                0x3333333333333333_u64,
                                0xCCCCCCCCCCCCCCCC_u64,
                            );
                            up0(
                                4,
                                0x0F0F0F0F0F0F0F0F_u64,
                                0xF0F0F0F0F0F0F0F0_u64,
                            );
                        }
                        {
                            let mut up1 = |i: u32, s: u32, l: u64, r: u64| {
                                if byte_width > i && (k & s > 0) {
                                    x = (((x as u64 & l).wrapping_shl(s))
                                         | ((x as u64 & r).wrapping_shr(s)))
                                as Self;
                                }
                            };

                            up1(
                                1,
                                8,
                                0x00FF00FF00FF00FF_u64,
                                0xFF00FF00FF00FF00_u64,
                            );
                            up1(
                                2,
                                16,
                                0x0000FFFF0000FFFF_u64,
                                0xFFFF0000FFFF0000_u64,
                            );
                            up1(
                                4,
                                32,
                                0x00000000FFFFFFFF_u64,
                                0xFFFFFFFF00000000_u64,
                            );
                        }
                        x
                    }
                }
            };
        }
    }
}

impl_all!(impl_rbit: u8, u16, u32, u64, i8, i16, i32, i64);

#[cfg(test)]
mod tests {
    use super::Rbit;

    #[test]
    fn rbit_u8() {
        (0..u8::max_value())
            .map(|x| {
                assert_eq!(x, x.rbit().rbit());
                let x = x as i8;
                assert_eq!(x, x.rbit().rbit());
            })
            .count();
    }
    #[test]
    fn rbit_u16() {
        (0..u16::max_value())
            .map(|x| {
                assert_eq!(x, x.rbit().rbit());

                let x = x as i16;
                assert_eq!(x, x.rbit().rbit());
            })
            .count();
    }
    #[test]
    fn rbit_u32() {
        (0..u32::max_value())
            .take(1000000)
            .map(|x| {
                assert_eq!(x, x.rbit().rbit());
                let x = x as i32;
                assert_eq!(x, x.rbit().rbit());
            })
            .count();
    }
    #[test]
    fn rbit_u64() {
        (0..u64::max_value())
            .take(1000000)
            .map(|x| {
                assert_eq!(x, x.rbit().rbit());
                let x = x as i64;
                assert_eq!(x, x.rbit().rbit());
            })
            .count();
    }

    #[test]
    fn rbit_tests() {
        {
            // width: 8
            let o_u8 = 0b1101_0011u8;
            let r_u8 = 0b1100_1011u8;
            assert_eq!(o_u8, 211);
            assert_eq!(r_u8, 203);
            assert_eq!(o_u8.rbit(), r_u8);
            let o_i8 = o_u8 as i8;
            let r_i8 = r_u8 as i8;
            assert_eq!(o_i8.rbit(), r_i8);
        }
        {
            //width: 16
            let o_u16 = 0b1101_0011_1110_1010u16;
            let r_u16 = 0b0101_0111_1100_1011u16;
            assert_eq!(o_u16.rbit(), r_u16);
            assert_eq!((o_u16 as i16).rbit(), r_u16 as i16);
        }
        {
            // width: 32
            let o_u32 = 0b1101_0011_1110_1010_1101_0011_1010_1010u32;
            let r_u32 = 0b0101_0101_1100_1011_0101_0111_1100_1011u32;
            assert_eq!(o_u32.rbit(), r_u32);
            assert_eq!((o_u32 as i32).rbit(), r_u32 as i32);
        }
        {
            // width: 64
            let o_u64 = 0b1101_0011_0010_1010_1111_0011_1010_1010_1101_0011_1110_1010_1101_0011_1010_1010u64;
            let r_u64 = 0b0101_0101_1100_1011_0101_0111_1100_1011_0101_0101_1100_1111_0101_0100_1100_1011u64;
            assert_eq!(o_u64.rbit(), r_u64);
            assert_eq!((o_u64 as i64).rbit(), r_u64 as i64);
        }
    }
}
