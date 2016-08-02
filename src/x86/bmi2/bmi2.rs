use int::IntF32T64;
use x86::bmi2::intrinsics;
use alg;

pub fn bzhi<T: IntF32T64>(x: T, bit_position: T) -> T {
    debug_assert!(bit_position < T::bit_size());
    if cfg!(target_feature = "bmi2") {
        unsafe { intrinsics::bzhi(x, bit_position) }
    } else {
        alg::bmi2::bzhi(x, bit_position)
    }
}

pub fn mulx<T: IntF32T64 + alg::bmi2::MULX>(x: T, y: T, p: &mut T) -> T {
    alg::bmi2::mulx(x, y, p)
}

pub fn pdep<T: IntF32T64>(x: T, mask_: T) -> T {
    if cfg!(target_feature = "bmi2") {
        unsafe { intrinsics::pdep(x, mask_) }
    } else {
        alg::bmi2::pdep(x, mask_)
    }
}

pub fn pext<T: IntF32T64>(x: T, mask_: T) -> T {
    if cfg!(target_feature = "bmi2") {
        unsafe { intrinsics::pext(x, mask_) }
    } else {
        alg::bmi2::pext(x, mask_)
    }
}
