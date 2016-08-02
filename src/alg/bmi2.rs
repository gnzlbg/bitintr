use int::Int;
use std::mem;

pub fn bzhi<T: Int>(x: T, bit_position: T) -> T {
    debug_assert!(bit_position < T::bit_size());
    x & ((T::one() << bit_position) - T::one())
}

pub trait MULX {
    fn mulx(self, Self, &mut Self) -> Self;
}

impl MULX for u8 {
    fn mulx(self, y: u8, p: &mut u8) -> u8 {
        let result: u16 = (self as u16) * (y as u16);
        *p = (result >> 8) as u8;
        result as u8
    }
}

impl MULX for u16 {
    fn mulx(self, y: u16, p: &mut u16) -> u16 {
        let result: u32 = (self as u32) * (y as u32);
        *p = (result >> 16) as u16;
        result as u16
    }
}

impl MULX for u32 {
    fn mulx(self, y: u32, p: &mut u32) -> u32 {
        let result: u64 = (self as u64) * (y as u64);
        *p = (result >> 32) as u32;
        result as u32
    }
}

impl MULX for u64 {
    fn mulx(self, y: u64, hi: &mut u64) -> u64 {
        // TODO: Blocking on 128bit integers
        //    let result: u128 = (x as u128) * (y as u128);
        //    *p = (result >> 64) as u64;
        //    result as u64
        let u1 = self & 0xffffffff;
        let v1 = y & 0xffffffff;
        let t = u1 * v1;
        let w3 = t & 0xffffffff;
        let k = t >> 32;

        let x = self >> 32;
        let t = (self * v1) + k;
        let k = t & 0xffffffff;
        let w1 = t >> 32;

        let y = y >> 32;
        let t = (u1 * y) + k;

        *hi = (x * y) + w1 + k;
        (t << 32) + w3
    }
}

impl MULX for usize {
    fn mulx(self, y: usize, p: &mut usize) -> usize {
        match mem::size_of::<usize>() * 8 {
            8 => (self as u8).mulx(y as u8, unsafe { mem::transmute(p) }) as usize,
            16 => (self as u16).mulx(y as u16, unsafe { mem::transmute(p) }) as usize,
            32 => (self as u32).mulx(y as u32, unsafe { mem::transmute(p) }) as usize,
            64 => (self as u64).mulx(y as u64, unsafe { mem::transmute(p) }) as usize,
            _ => unreachable!(),
        }
    }
}

impl MULX for i8 {
    fn mulx(self, y: i8, p: &mut i8) -> i8 {
        (self as u8).mulx(y as u8, unsafe { mem::transmute(p) }) as i8
    }
}

impl MULX for i16 {
    fn mulx(self, y: i16, p: &mut i16) -> i16 {
        (self as u16).mulx(y as u16, unsafe { mem::transmute(p) }) as i16
    }
}

impl MULX for i32 {
    fn mulx(self, y: i32, p: &mut i32) -> i32 {
        (self as u32).mulx(y as u32, unsafe { mem::transmute(p) }) as i32
    }
}

impl MULX for i64 {
    fn mulx(self, y: i64, p: &mut i64) -> i64 {
        (self as u64).mulx(y as u64, unsafe { mem::transmute(p) }) as i64
    }
}


impl MULX for isize {
    fn mulx(self, y: isize, p: &mut isize) -> isize {
        (self as usize).mulx(y as usize, unsafe { mem::transmute(p) }) as isize
    }
}



/// Unsigned multiplication of `x` with `y` storing the high half of the result
/// in `p` and returning the lower half of the result.
///
pub fn mulx<T: MULX>(x: T, y: T, hi: &mut T) -> T {
    x.mulx(y, hi)
}

/// Paralel bits deposit
pub fn pdep<T: Int>(x: T, mask_: T) -> T {
    let mut res = T::zero();
    let mut mask = mask_;
    let mut bb = T::one();
    loop {
        if mask == T::zero() {
            break;
        }
        if (x & bb) != T::zero() {
            res |= mask & mask.wrapping_neg();
        }
        mask &= mask - T::one();
        bb += bb;
    }
    res
}

/// Parallel bits extract
pub fn pext<T: Int>(x: T, mask_: T) -> T {
    let mut res = T::zero();
    let mut mask = mask_;
    let mut bb = T::one();
    loop {
        if mask == T::zero() {
            break;
        }
        if x & mask & (mask.wrapping_neg()) != T::zero() {
            res |= bb;
        }
        mask &= mask - T::one();
        bb += bb;
    }
    res
}
