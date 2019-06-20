//! rev

/// Byte reverse
pub trait Rev {
    /// Reverse the order of the bytes.
    ///
    /// **Keywords**: count leading sign bits, count leading ones, count
    /// leading bits set.
    ///
    /// # Instructions
    ///
    /// - [`REV`](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.
    ///   ddi0419c/index.html):
    ///   - Description: Reverses the byte order in a register.
    ///   - Architecture: ARMv6, ARMv7, ARMv8.
    ///   - Registers: 32 (v6, v7)/64 (v8) bits.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// let n = 0b1111_1111_1100_1010u16;
    /// let m = 0b1100_1010_1111_1111u16;
    ///
    /// assert_eq!(n.rev(), m);
    /// assert_eq!(m.rev(), n);
    /// ```
    fn rev(self) -> Self;
}

macro_rules! impl_rev {
    ($id:ident) => {
        impl Rev for $id {
            #[inline]
            fn rev(self) -> Self {
                self.swap_bytes()
            }
        }
    };
}

impl_all!(impl_rev: u8, u16, u32, u64, i8, i16, i32, i64);
