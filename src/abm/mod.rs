//! Advanced Bit Manipulation (ABM) instruction set.
//!
//! For a quick overview see [wikipedia](). The reference is [Intel 64 and IA-32
//! Architectures Software Developer's Manual Volume 2: Instruction Set
//! Reference,
//! A-Z](http://www.intel.de/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf).
//!
//! It consist of the following two instructions that operate on 16/32/64bit
//! registers:
//!
//! - `POPCNT` (note: Intel cosiders it part of SSE4.2 but advertises it with
//!    its own CPUID flag).
//! - `LZCNT` (note: it is _officially_ part of BMI1, but Intel CPUs advertise
//!    it as ABM).

mod popcnt;
mod lzcnt;

pub use abm::popcnt::*;
pub use abm::lzcnt::*;
