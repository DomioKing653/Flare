#!/bin/bash
# This script is used to safely build releases for
# github. It's not used for actually installing Vertex
cargo clean
cargo update
cargo build --release
cd src/runtime_lib|| exit
cargo build --lib --release || echo "!Build failed!" && exit
echo "vertexC and vertex are at ./target/release and vm is at src/codegen/target/release/libvm_runtime.a"
