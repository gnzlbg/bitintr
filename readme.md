# Bitwise Intrinsics

This crate exposes the Bitwise Manipulation Intrinsics offered by modern CPU
instruction sets. Three different ways of accessing the intrinsics are available:

- `bitintr::alg::*`: portable software emulation of all intrinsics.
- `bitintr::x86::*`: intrinsics for the types offered by the instruction sets
   (fallbacks to software emulation if the CPU does not support the intrinsic).
- `bitintr::*`: generic intrinsics for all integer types (use CPU intrinsics
when avaialbe in the CPU for the types involved).

The following instruction sets are exposed:

- `abm`: Advanced Bit Manipulation instructions.
- `tbm`: Trailing Bit Manipulation instructions.
- `bmi`: Bit Manipulation Instruction Set.
- `bmi2`: Bit Manipulation Instruction Set 2.

All intrinsics are named after their CPU instruction.
