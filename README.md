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

## Working on;
    [Y] Gnome
    [N] Kde
    [?] MacOS
    [?] Windows
    