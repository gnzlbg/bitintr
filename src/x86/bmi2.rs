//! Bit Manipulation Instruction (BMI) Set 2.0.
//!
//! The reference is [Intel 64 and IA-32 Architectures Software Developer's
//! Manual Volume 2: Instruction Set Reference,
//! A-Z](http://www.intel.de/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf).
//!
//! [Wikipedia](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI2_.28Bit_Manipulation_Instruction_Set_2.29)
//! provides a quick overview of the available instructions.

pub use alg::x86::bmi2::mulx::*;
pub use super::intrinsics::bmi2::bzhi::*;
pub use super::intrinsics::bmi2::pdep::*;
pub use super::intrinsics::bmi2::pext::*;
