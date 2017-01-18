//! Advanced Bit Manipulation (ABM) instruction set.
//!
//! For a quick overview see
//! [wikipedia](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#ABM_.28Advanced_Bit_Manipulation.29).
//! The references are [Intel 64 and IA-32 Architectures Software Developer's
//! Manual Volume 2: Instruction Set Reference,
//! A-Z](http://www.intel.de/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf)
//! and  [AMD64 Architecture Programmer's Manual, Volume 3: General-Purpose and System Instructions](http://support.amd.com/TechDocs/24594.pdf).
//!
//! It consist of the following two instructions that operate on 16/32/64bit
//! registers:
//!
//! - `POPCNT` (note: Intel cosiders it part of SSE4.2 but advertises it with
//!    its own CPUID flag AMD advertises it as ABM).
//! - `LZCNT` (note: it is _officially_ part of BMI1, but Intel and AMD CPUs
//!    advertise it as ABM).

pub use alg::abm::*;
