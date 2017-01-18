//! Bit Manipulation Instruction (BMI) Set 1.0: software fallback.

pub mod andn;
pub mod bextr;
pub mod bextri;
pub mod blsi;
pub mod blsmsk;
pub mod blsr;
pub mod tzcnt;

pub use alg::bmi::andn::*;
pub use alg::bmi::bextr::*;
pub use alg::bmi::bextri::*;
pub use alg::bmi::blsi::*;
pub use alg::bmi::blsmsk::*;
pub use alg::bmi::blsr::*;
pub use alg::bmi::tzcnt::*;
