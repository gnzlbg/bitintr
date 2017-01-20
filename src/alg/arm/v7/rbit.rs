use int::Int;

/// Reverse the bit order.
///
/// # Assembly Instructions
///
/// - [`RBIT`][http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0487a.k_10775/index.html]:
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
/// assert_eq!(rbit(0b11111111u32), 0b11111111_00000000_00000000_00000000u32);
/// ```
pub fn rbit<T: Int>(y: T) -> T {
    let mut x = y;
    let width  = T::byte_size();
    let k = T::bit_size() - T::from_u32(1);

    {
        let mut up0 = |i, l, r| {
            if k & i > T::from_u32(0) {
                x = ((x & l) << i)
                    | ((x & r) >> i);
            }
        };

        up0(T::from_u32(1), T::from_u64(0x5555555555555555u64), T::from_u64(0xAAAAAAAAAAAAAAAAu64));
        up0(T::from_u32(2), T::from_u64(0x3333333333333333u64), T::from_u64(0xCCCCCCCCCCCCCCCCu64));
        up0(T::from_u32(4), T::from_u64(0x0F0F0F0F0F0F0F0Fu64), T::from_u64(0xF0F0F0F0F0F0F0F0u64));
    }
    {
        let mut up1 = |i, s, l, r| {
            if width > i && (k & s > T::from_u32(0)) {
                x = ((x & l) << s)
                    | ((x & r) >> s);
            }
        };

        up1(T::from_u32(1), T::from_u32(8), T::from_u64(0x00FF00FF00FF00FFu64), T::from_u64(0xFF00FF00FF00FF00u64));
        up1(T::from_u32(2), T::from_u32(16), T::from_u64(0x0000FFFF0000FFFFu64), T::from_u64(0xFFFF0000FFFF0000u64));
        up1(T::from_u32(4), T::from_u32(32), T::from_u64(0x00000000FFFFFFFFu64), T::from_u64(0xFFFFFFFF00000000u64));
    }
    x
}

/// Method version of [`rbit`](fn.rbit.html).
pub trait RBIT {
    fn rbit(self) -> Self;
}

impl<T: Int> RBIT for T {
    fn rbit(self) -> T {
        rbit(self)
    }
}
