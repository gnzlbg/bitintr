# X86, X86_64, AMD64 intrinsics

The `intrinsics` module abstracts the compiler hooks for calling the intrinsics
directly.

The instruction set modules (`abm`, `tbm`, `bmi`, `bmi2`) export the target
specific intrinsics with software fallback.
