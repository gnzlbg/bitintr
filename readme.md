# Portable Bitwise Manipulation Intrinsics

[![crates.io version][crate-shield]][crate] [![Travis build status][travis-shield]][travis] [![Coveralls.io code coverage][coveralls-shield]][coveralls] [![Docs][docs-shield]][docs] [![License][license-shield]][license]

> `0b0000_0010_1001_1010`

The intrinsics are named after their CPU instruction and organized in modules
named after their architecture/instruction set:
`bitintr::{arch}::{instruction_set}::{intrinsic_name}`.

They are implemented for all integer types _except_ `u128/i128`. Whether a
fallback software implementation is used depends on the integer types involved
and the instruction sets supported by the target.

The following instruction sets are implemented:

- x86 (`bitintr::x86`):
  - [`ABM`][abm_link]: Advanced Bit Manipulation instructions ([`bitintr::x86::abm`](https://gnzlbg.github.io/bitintr/bitintr/x86/abm/index.html)).
  - [`TBM`][tbm_link]: Trailing Bit Manipulation instructions ([`bitintr::x86::tbm`](https://gnzlbg.github.io/bitintr/bitintr/x86/tbm/index.html)).
  - [`BMI`][bmi1_link]: Bit Manipulation Instruction Set 1.0 ([`bitintr::x86::bmi`](https://gnzlbg.github.io/bitintr/bitintr/x86/bmi/index.html)).
  - [`BMI2`][bmi2_link]: Bit Manipulation Instruction Set 2.0 ([`bitintr::x86::bmi2`](https://gnzlbg.github.io/bitintr/bitintr/x86/bmi2/index.html)).

- ARM (`bitintr::arm`):
  - [`ARMv5`][armv5_link]: [`bitintr::arm::v5`](https://gnzlbg.github.io/bitintr/bitintr/arm/v5/index.html).
  - [`ARMv6`][armv6_link]: [`bitintr::arm::v6`](https://gnzlbg.github.io/bitintr/bitintr/arm/v6/index.html).
  - [`ARMv7`][armv7_link]: [`bitintr::arm::v7`](https://gnzlbg.github.io/bitintr/bitintr/arm/v7/index.html).
  - [`ARMv8`][armv8_link]: [`bitintr::arm::v8`](https://gnzlbg.github.io/bitintr/bitintr/arm/v8/index.html).

**Note**: This library is low-level by purpose. For a portable higher-level
bitwise manipulation algorithms library you might want to check out
the [bitwise][bitwise_link] crate.

## Example

```rust
extern crate bitintr;
use bitintr::x86::bmi2::*;

fn main() {
    // Intrinsics are provided as trait methods:
    let method_call = 1.pdep(0);
    // And as free functions:
    let free_call = pdep(1, 0);
    assert_eq!(method_call, free_call);
}
```

## Supported compilers

> The minimum required rustc version is >= **1.13.0**.

When compiled with a rust stable compiler the intrinsics are implemented using
the software fallback. In release builds LLVM _often_ generates the corresponding
CPU instruction.

When compiled with a rust nightly compiler the following unstable features are
used to generate the corresponding CPU instruction in _all_ cases:

- [`cfg_target_feature`][cfg_target_feature] for target-dependent behavior,
- [`platform_intrinsics`][platform_intrinsics_feature] for using the bitwise manipulation compiler intrinsics,
- [`link_llvm_intrinsics`][link_llvm_intrinsics_feature] for using the llvm intrinsics not yet available in rustc, and
- [`i128_type`][i128_type_feature] support for _efficient_ 64-bit multiplication (using `u128`).

## License

Licensed under the [MIT license][license].

## Contribution

Yes please! Just note that all contributions shall be licensed as above without
any additional terms or conditions.

<!-- Links -->
[travis-shield]: https://img.shields.io/travis/gnzlbg/bitintr.svg?style=flat-square
[travis]: https://travis-ci.org/gnzlbg/bitintr
[coveralls-shield]: https://img.shields.io/coveralls/gnzlbg/bitintr.svg?style=flat-square
[coveralls]: https://coveralls.io/github/gnzlbg/bitintr
[docs-shield]: https://img.shields.io/badge/docs-online-blue.svg?style=flat-square
[docs]: https://gnzlbg.github.io/bitintr
[license-shield]: https://img.shields.io/github/license/mashape/apistatus.svg?style=flat-square
[license]: https://github.com/gnzlbg/bitintr/blob/master/license.md
[crate-shield]: https://img.shields.io/crates/v/bitintr.svg?style=flat-square
[crate]: https://crates.io/crates/bitintr
[abm_link]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#ABM_.28Advanced_Bit_Manipulation.29
[tbm_link]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#TBM_.28Trailing_Bit_Manipulation.29
[bmi1_link]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI1_.28Bit_Manipulation_Instruction_Set_1.29
[bmi2_link]: https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#BMI2_.28Bit_Manipulation_Instruction_Set_2.29
[armv5_link]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0100i/index.html
[armv6_link]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0419c/index.html
[armv7_link]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0403e.b/index.html
[armv8_link]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0487a.k_10775/index.html
[cfg_target_feature]: https://github.com/rust-lang/rust/issues/29717
[platform_intrinsics_feature]: https://doc.rust-lang.org/book/intrinsics.html
[i128_type_feature]: https://github.com/rust-lang/rust/issues/35118
[link_llvm_intrinsics_feature]: https://github.com/rust-lang/rust/issues/29602
[bitwise_link]: https://github.com/gnzlbg/bitwise
