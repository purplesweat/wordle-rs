#!/bin/sh

if command -v cargo
then
    # We have cargo, that is good
    cargo build --release
    mkdir -p /usr/share/dict
    cp assets/words.sorted /usr/share/dict
    cp target/release/wordle ~/.cargo/bin
else
    # I have not an idea how to compile the random crate...
    echo "cargo doesn't seem to be installed, try"
    echo "$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi
