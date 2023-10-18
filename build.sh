#!/bin/bash

cargo build --release --features "lua54" --target-dir=target/lua54
cargo build --release --features "lua53" --target-dir=target/lua53
cargo build --release --features "lua52" --target-dir=target/lua52
cargo build --release --features "lua51" --target-dir=target/lua51
