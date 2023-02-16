// https://lib.rs/crates/arboard
// https://crates.io/crates/magic-crypt
// https://docs.rs/enigo/latest/enigo/
// https://github.com/enigo-rs/enigo
use enigo::*;
use std::{fs, thread};
use std::time::Duration;
use magic_crypt::MagicCryptTrait;


fn main() {
    let path: &str = "/home/foxx/.sekret";

    let data = match fs::read_to_string(path) {
        Ok(x) => x,
        Err(_) => "ERR".to_string(),
    };

    // let mc = magic_crypt::new_magic_crypt!("scrumdiddlyumptious", 256);

    // let decrypted = mc.decrypt_base64_to_string(data).unwrap();
    // println!("{}", decrypted);

    let mut enigo = Enigo::new();
    thread::sleep(Duration::new(1,0));
    enigo.key_sequence(data.as_str());
}
