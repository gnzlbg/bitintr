//! Bit Manipulation Instruction (BMI) Set 2.
mod bzhi;
mod pdep;
mod pext;

pub use x86::bmi2::bzhi::bzhi;
pub use x86::bmi2::pdep::pdep;
pub use x86::bmi2::pext::pext;
