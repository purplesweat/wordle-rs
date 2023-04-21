#!/bin/sh

if command -v cargo
then
    # We have cargo, that is good
    cargo build --release
    mkdir -p /usr/share/dict
    su_cmd=
    if command -v sudo
    then
        su_cmd=sudo
    else if command -v doas
    then
        su_cmd=doas
    else
        su_cmd=su -c "$@"
    fi
    $su_cmd cp assets/words.sorted /usr/share/dict/words.sorted
    $su_cmd cp target/release/wordle-rs $HOME/.cargo/bin/wordle-rs
else
    # I have not an idea how to compile the random crate...
    echo "cargo doesn't seem to be installed, try"
    echo "$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi
