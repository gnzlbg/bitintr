# Module structure

The library is split into multiple modules:

- `alg::{arch}::{instruction_set}::{instruction}`: _Portable software emulation_
  of all the intrinsics for all integer types.

The following modules export the intrinsics for each architecture following the
`{arch}::{instruction_set}` convention:

- `x86`.
- `arm::v{5|6|7|8}`.
