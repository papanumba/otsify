#!/bin/sh
cargo build --release && cp target/release/otsify bin/linux64/
cargo build --release --target=x86_64-pc-windows-gnu && \
cp target/x86_64-pc-windows-gnu/release/otsify.exe bin/win64/
