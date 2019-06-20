//! cls

/// Count leading sign bits
pub trait Cls {
    /// Counts the leading most significant bits set.
    ///
    /// When all bits of the operand are set it returns the size of the
    /// operand in bits.
    ///
    /// **Keywords**: count leading sign bits, count leading ones, count
    /// leading bits set.
    ///
    /// # Instructions
    ///
    /// - [`CLS`](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.
    ///   ddi0487a.k_10775/index.html):
    ///   - Description: Count leading sign bits.
    ///   - Architecture: ARMv8.
    ///   - Registers: 32/64 bits.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// let n = 0b1111_1111_1100_1010_u16;
    /// assert_eq!(n.cls(), 9);
    /// assert_eq!(0b1111_1111_u8.cls(), 7);
    /// ```
    fn cls(self) -> Self;
}

macro_rules! impl_cls {
    ($id:ident, $sid:ident, $width:expr) => {
        #[allow(clippy::use_self)]
        impl Cls for $id {
            #[inline]
            fn cls(self) -> Self {
                Self::leading_zeros(
                    (((((self as $sid) >> ($width - 1)) as Self) ^ self) << 1)
                        | 1,
                ) as Self
            }
        }
    };
}

impl_cls!(u8, i8, 8);
impl_cls!(i8, i8, 8);
impl_cls!(u16, i16, 16);
impl_cls!(i16, i16, 16);
impl_cls!(u32, i32, 32);
impl_cls!(i32, i32, 32);
impl_cls!(u64, i64, 64);
impl_cls!(i64, i64, 64);
