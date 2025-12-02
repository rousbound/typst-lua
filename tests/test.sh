#!/bin/bash
ln -s ~/.cargo/target/debug/libtypst.so typst.so
lua test.lua
