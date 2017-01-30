//! Bit Manipulation Instruction (BMI) Set 1.0.
//!
//! The reference is [Intel 64 and IA-32 Architectures Software Developer's
//! Manual Volume 2: Instruction Set Reference,
//! A-Z](http://www.intel.de/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf).
//!
//! [Wikipedia](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI1_.28Bit_Manipulation_Instruction_Set_1.29)
//! provides a quick overview of the available instructions.

pub use super::intrinsics::bmi::bextr::*;
pub use alg::x86::bmi::andn::*;
pub use alg::x86::bmi::blsi::*;
pub use alg::x86::bmi::blsmsk::*;
pub use alg::x86::bmi::blsr::*;
pub use alg::x86::bmi::tzcnt::*;
