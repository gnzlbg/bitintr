//! Bit Manipulation Instruction (BMI) Set 2.0.
//!
//! For a quick overview see
//! [wikipedia](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI2_.28Bit_Manipulation_Instruction_Set_2.29).
//! The reference is [Intel 64 and IA-32 Architectures Software Developer's
//! Manual Volume 2: Instruction Set Reference,
//! A-Z](http://www.intel.de/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf).

mod bzhi;
mod pdep;
mod pext;

pub use bmi2::bzhi::*;
pub use alg::bmi2::mulx::*;
pub use bmi2::pdep::*;
pub use bmi2::pext::*;
