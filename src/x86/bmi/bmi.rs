use int::{IntF16T64, IntF32T64};
use x86::bmi::intrinsics;
use alg;

pub fn andn<T: IntF32T64>(x: T, y: T) -> T {
    alg::bmi::andn(x, y)
}

pub fn bextr<T: IntF32T64>(source: T, start: T, length: T) -> T {
    if cfg!(target_feature = "bmi") {
        unsafe { intrinsics::bextr(source, start, length) }
    } else {
        alg::bmi::bextr(source, start, length)
    }
}

/// BEXTR AMD version (TODO: document)
pub fn bextri<T: IntF32T64>(source: T, range: T) -> T {
    if cfg!(target_feature = "bmi") {
        unsafe { intrinsics::bextri(source, range) }
    } else {
        alg::bmi::bextri(source, range)
    }
}

pub fn blsi<T: IntF32T64>(x: T) -> T {
    alg::bmi::blsi(x)
}

pub fn blsmsk<T: IntF32T64>(x: T) -> T {
    alg::bmi::blsmsk(x)
}

pub fn blsr<T: IntF32T64>(x: T) -> T {
    alg::bmi::blsr(x)
}

pub fn tzcnt<T: IntF16T64>(x: T) -> T {
    x.trailing_zeros()
}
