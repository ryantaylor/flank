#!/bin/bash

cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-gnu
cross build --release --target aarch64-apple-darwin

tar -czvf x86_64-unknown-linux-gnu.tar.gz -C target/x86_64-unknown-linux-gnu/release flank
tar -czvf x86_64-pc-windows-gnu.tar.gz -C target/x86_64-pc-windows-gnu/release flank.exe
tar -czvf aarch64-apple-darwin.tar.gz -C target/aarch64-apple-darwin/release flank
