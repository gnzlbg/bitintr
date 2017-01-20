# Module structure

TODO

The library is split into multiple modules:

- `alg`: _Portable software emulation_ of all the intrinsics for all integer types.

- _Platform specific intrinsics_: only offered for the types for which the
  intrinsics actually map to a CPU instruction such that trying to use an
  intrinsic on a type that cannot support it produces a compilation error. If
  the CPU does not support the intrinsic these fall back to software emulation.
  Platforms available:
  - `x86`.
  - `arm::v{6|7|8}`.

- _Generic intrinsics_: these work for all integer types. These use CPU intrinsics
  when available for the particular integer type and CPU, and fall back to
  software otherwise.
  - `abm/`: Advanced Bit Manipulation intrinsics.
  - `bmi/`: Bit Manipulation Instruction Set intrinsics.
  - `bmi2/`: Bit Manipulation Instruction Set 2 intrinsics.
  - `tbm/`: Trailing Bit Manipulation intrinsics.
