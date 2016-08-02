# Module structure

The library is split into multiple modules:

- `alg`: Portable software emulation of all the intrinsics for all integer types.

- Platform specific intrinsics. These are only offered for the types for which
  the intrinsics actually map to a CPU instruction. If the CPU does not support
  the intrinsic these fall back to software emulation. Platforms available:
  - `x86`.

- Generic intrinsics: these work for all integer types. These use CPU intrinsics
  when available for the particular integer type and CPU, and fall back to
  software otherwise.
  - `abm/`: Advanced Bit Manipulation intrinsics.
  - `bmi/`: Bit Manipulation Instruction Set intrinsics.
  - `bmi2/`: Bit Manipulation Instruction Set 2 intrinsics.
  - `tbm/`: Trailing Bit Manipulation intrinsics.

You probably just want to use the generic intrinsics since they are a
best-effort solution. When you want to be sure that a particular intrinsic will
be used you can use the platform specific ones which only work on types for
which the intrinsics exist.
