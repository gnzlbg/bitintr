//! BMI 1 Intrinsics

use int::IntF32T64;
use std::mem::size_of;

#[allow(dead_code)]
extern "platform-intrinsic" {
    fn x86_bmi_bextr_32(x: u32, y: u32) -> u32;
    fn x86_bmi_bextr_64(x: u64, y: u64) -> u64;
}

pub unsafe fn bextri<T: IntF32T64>(source: T, range: T) -> T {
    match size_of::<T>() * 8 {
        32 => T::from_u32(x86_bmi_bextr_32(source.to_u32(), range.to_u32())),
        64 => T::from_u64(x86_bmi_bextr_64(source.to_u64(), range.to_u64())),
        _ => unreachable!(),
    }
}

pub unsafe fn bextr<T: IntF32T64>(source: T, start: T, length: T) -> T {
    match size_of::<T>() * 8 {
        32 => {
            bextri(source,
                   ((start & T::from_u32(0xff)) | ((length & T::from_u32(0xff)) << T::from_u32(8))))
        }
        64 => {
            bextri(source,
                   ((start & T::from_u64(0xff)) | ((length & T::from_u64(0xff)) << T::from_u64(8))))
        }
        _ => unreachable!(),
    }
}
