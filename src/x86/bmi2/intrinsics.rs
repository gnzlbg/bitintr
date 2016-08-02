//! BMI 2 Intrinsics

use int::IntF32T64;
use std::mem::size_of;

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn x86_bmi2_bzhi_32(x: u32, y: u32) -> u32;
    fn x86_bmi2_bzhi_64(x: u64, y: u64) -> u64;

    fn x86_bmi2_pdep_32(x: u32, y: u32) -> u32;
    fn x86_bmi2_pdep_64(x: u64, y: u64) -> u64;

    fn x86_bmi2_pext_32(x: u32, y: u32) -> u32;
    fn x86_bmi2_pext_64(x: u64, y: u64) -> u64;
}

pub unsafe fn bzhi<T: IntF32T64>(x: T, y: T) -> T {
    match size_of::<T>() * 8 {
        32 => T::from_u32(x86_bmi2_bzhi_32(x.to_u32(), y.to_u32())),
        64 => T::from_u64(x86_bmi2_bzhi_64(x.to_u64(), y.to_u64())),
        _ => unreachable!(),
    }
}

pub unsafe fn pdep<T: IntF32T64>(x: T, y: T) -> T {
    match size_of::<T>() * 8 {
        32 => T::from_u32(x86_bmi2_pdep_32(x.to_u32(), y.to_u32())),
        64 => T::from_u64(x86_bmi2_pdep_64(x.to_u64(), y.to_u64())),
        _ => unreachable!(),
    }
}

pub unsafe fn pext<T: IntF32T64>(x: T, y: T) -> T {
    match size_of::<T>() * 8 {
        32 => T::from_u32(x86_bmi2_pext_32(x.to_u32(), y.to_u32())),
        64 => T::from_u64(x86_bmi2_pext_64(x.to_u64(), y.to_u64())),
        _ => unreachable!(),
    }
}
