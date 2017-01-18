# Bitwise Manipulation Intrinsics

[![Travis build status][travis-shield]][travis] [![Coveralls.io code coverage][coveralls-shield]][coveralls] [![Docs][docs-shield]][docs] [![License][license-shield]][license]

> Portable bitwise manipulation instructions (**nightly-only**)

Portable implementation of bitwise manipulation instructions. The intrinsics
are:

- named after the corresponding CPU instruction, 
- organized in instruction set modules:
  `bitintr::{instruction_set}::{intrinsic}`, and
- implemented for all integer types, with software fallback depending on the
  integer type and the instruction sets supported by the target.

The following instruction sets are implemented:

- [`ABM`][abm_link]: Advanced Bit Manipulation instructions.
- [`TBM`][tbm_link]: Trailing Bit Manipulation instructions.
- [`BMI`][bmi1_link]: Bit Manipulation Instruction Set 1.0.
- [`BMI2`][bmi2_link]: Bit Manipulation Instruction Set 2.0.

## Example

```rust
use bitintr::bmi2::pdep;
fn main() {
  let x = 1;
  let y = 0;
  // Intrinsics can be used as methods:
  let method_call = x.pdep(y);
  // And as free function calls:
  let free_call = pdep(x, y);
  assert_eq!(method_call, free_call);
}
```

## Nightly-only

This crate relies on the following nightly features:

- `specialization` for selecting better algorithms for
different sets of integer types,
- `cfg_target_feature` for target-dependent behavior,
- `platform_intrinsics` for using the bitwise manipulation intrinsics, and
- `u128` support for efficient lossless unsigned 64bit multiplication.

For more information, visit the [documentation page][docs].

<!-- Links -->
[travis-shield]: https://img.shields.io/travis/gnzlbg/bitintr.svg?style=flat-square
[travis]: https://travis-ci.org/gnzlbg/bitintr
[coveralls-shield]: https://img.shields.io/coveralls/gnzlbg/bitintr.svg?style=flat-square
[coveralls]: https://coveralls.io/github/gnzlbg/bitintr
[docs-shield]: https://img.shields.io/badge/docs-online-blue.svg?style=flat-square
[docs]: https://gnzlbg.github.io/bitintr
[license-shield]: https://img.shields.io/github/license/mashape/apistatus.svg?style=flat-square
[license]: https://github.com/gnzlbg/bitintr/blob/master/LICENSE
[abm_link]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#ABM_.28Advanced_Bit_Manipulation.29
[tbm_link]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#TBM_.28Trailing_Bit_Manipulation.29
[bmi1_link]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI1_.28Bit_Manipulation_Instruction_Set_1.29
[bmi2_link]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI2_.28Bit_Manipulation_Instruction_Set_2.29
