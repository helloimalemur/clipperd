# Clipperd

- Acts like programmable "G" keys
- Clipboards are encrypted in memory
- Will not output when screen is locked
## How to use
    1. [Control] + [C], copy to normal clipboard
    2. [L-Control] +[L-Shift] + [F1-F4], will copy value from clipboard to (encrypted) memory
    3. [L-Control] +[L-Shift] + [L-Alt] + [F1-F5], will write value from (encrypted) memory to clipboard

## Installation
    cargo install clipperd

## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check


# Resources
https://github.com/Enigo-rs/Enigo
https://github.com/Narsil/rdev
https://github.com/ItayGarin/ktrl
https://lib.rs/crates/os_info
https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
https://lib.rs/crates/arboard
https://crates.io/crates/magic-crypt
https://docs.rs/enigo/latest/enigo/
https://github.com/enigo-rs/enigo
