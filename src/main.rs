// https://lib.rs/crates/arboard
// https://github.com/obv-mikhail/InputBot
// https://docs.rs/enigo/latest/enigo/
// https://github.com/enigo-rs/enigo
use arboard::Clipboard;
use enigo::*;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    println!("{}", clipboard.get().text().unwrap());

    let mut enigo = Enigo::new();
    enigo.key_sequence("asfd");


}
