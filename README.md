# clipperd

- write modifier - acts like programmable "G" keys
- clipboards saved encrypted
- will not output when screen is locked

--Currently writes encrypted clipboard files to '/var/lib/clippard/.board{}'--

--/var/lib/clippard/ must be created and permissions set for user clippard will be run as--
#
    1. [Control] + [C], copy to normal clipboard
    2. [shift] + [F1-F5], will copy value from clipboard to encrypted file (/tmp/board[1-5])
    3. [control] + [F1-F5], will write value from encrypted file to keyboard output


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
