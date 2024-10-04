# Clipperd

- Secondary clipboard
- Clipboards are encrypted in memory, clipperd randomly generates encryption key on launch.
- Will not output when screen is locked
- AES256 encryption
- Retains formatting

## Usage Flow
    copy value to OS clipboard normally: [L-Control] + [c] 
    copy value from OS clipboard to (encrypted) memory: [L-Control] + [L-Shift] + [F1-F4]
    retreive value from (encrypted) memory to OS clipboard: [L-Control] + [L-Shift] + [L-Alt] + [F1-F5]
    paste value from OS clipboard normally: [L-Control] + [v]

## Install [Rust](https://www.rust-lang.org/tools/install)
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## Installation [requires Rust](https://www.rust-lang.org/tools/install)
    cargo install clipperd

## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check

## Working on;
    [Y] Gnome
    [N] Kde
    [?] MacOS
    [?] Windows
    