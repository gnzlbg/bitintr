//! Bit Manipulation Instruction (BMI) Set 2.0: software fallback.

pub mod bzhi;
pub mod mulx;
pub mod pdep;
pub mod pext;

pub use self::bzhi::*;
pub use self::mulx::*;
pub use self::pdep::*;
pub use self::pext::*;
