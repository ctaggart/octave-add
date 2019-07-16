#!/bin/sh -e
cargo build --release
cp target/release/liboctave_add.so src/add.oct