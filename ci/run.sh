#!/usr/bin/env bash

set -ex

export RUST_TEST_THREADS=1
export RUST_BACKTRACE=1
export RUST_TEST_NOCAPTURE=1
export OPT="--target=$TARGET"
export OPT_RELEASE="--release ${OPT}"

# Select cargo command: use cross by default
export CARGO_CMD=cross

# On Appveyor (windows) and Travis (x86_64-unknown-linux-gnu and apple) native targets we use cargo (no need to cross-compile):
if [[ $TARGET = *"windows"* ]] || [[ $TARGET == "x86_64-unknown-linux-gnu" ]] || [[ $TARGET = *"apple"* ]]; then
    export CARGO_CMD=cargo
fi

# Install cross if necessary:
if [[ $CARGO_CMD == "cross" ]]; then
    cargo install cross
fi

# Make sure TARGET is installed when using cargo:
if [[ $CARGO_CMD == "cargo" ]]; then
    rustup target add $TARGET || true
fi

$CARGO_CMD test $OPT_ND
! find target/ -name *.rlib -exec nm {} \; | grep "std"
$CARGO_CMD clean
$CARGO_CMD test $OPT_RELEASE_ND
! find target/ -name *.rlib -exec nm {} \; | grep "std"

if [[ $TARGET = *"x86"* ]]; then
    RUSTFLAGS="-C target-feature=+bmi,+bmi2" $CARGO_CMD test $OPT_RELEASE_ND
fi

if [[ $TARGET = *"armv7"* ]]; then
    RUSTFLAGS="-C target-feature=+neon" $CARGO_CMD test $OPT_RELEASE_ND
fi
