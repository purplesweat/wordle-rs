# wordle-rs
Wordle in the terminal... *but in Rust*... yay?

## Building
I built a script for it...
`$ sudo ./install.sh`

It checks dependencies. It works fine.

### Manually
***Alright, I get it, you're a masochist***
Assuming you have cargo installed:
`$ cargo build --release`
Move assets/words.sorted to /usr/share/dict/words.sorted
`$ mkdir -p /usr/share/dict`
`$ mv assets/words.sorted /usr/share/dict/words.sorted`
Install if you'd like:
`$ mv target/release/wordle ~/.cargo/bin`
