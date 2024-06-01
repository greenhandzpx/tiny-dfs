#!/bin/bash

# Run from root dir
export RUST_LOG=debug
cd core
cargo run storage 33333 44444 22222 /tmp/tiny-dfs