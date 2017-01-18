//! Bit Manipulation Instruction (BMI) Set 2.0: software fallback.

pub mod bzhi;
pub mod mulx;
pub mod pdep;
pub mod pext;

pub use alg::bmi2::bzhi::*;
pub use alg::bmi2::mulx::*;
pub use alg::bmi2::pdep::*;
pub use alg::bmi2::pext::*;
