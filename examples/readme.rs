//! Example of the top-level readme

extern crate bitintr;
use bitintr::bmi2::*;

fn main() {
    let x = 1;
    let y = 0;
    // Intrinsics can be used as methods:
    let method_call = x.pdep(y);
    // And as free function calls:
    let free_call = pdep(x, y);
    assert_eq!(method_call, free_call);
}
