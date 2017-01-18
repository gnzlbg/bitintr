//! Bit Manipulation Instruction (BMI) Set 2.0: software fallback.

mod bzhi;
mod mulx;
mod pdep;
mod pext;

pub use alg::bmi2::bzhi::*;
pub use alg::bmi2::mulx::*;
pub use alg::bmi2::pdep::*;
pub use alg::bmi2::pext::*;
