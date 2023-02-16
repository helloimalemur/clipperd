// https://lib.rs/crates/arboard
// https://crates.io/crates/magic-crypt
// https://docs.rs/enigo/latest/enigo/
// https://github.com/enigo-rs/enigo
use enigo::*;
use std::{fs, thread};
use std::time::Duration;
use base64::Engine;
use base64::engine::general_purpose;
use magic_crypt::MagicCryptTrait;
mod manage_clipboard;


fn main() {
    let path: &str = "/home/foxx/.sekret";
    let path2: &str = "/home/foxx/.sekret_enc";

    let data = match fs::read_to_string(path) {
        Ok(x) => x,
        Err(_) => "ERR".to_string(),
    };
    // let encoded: String = general_purpose::STANDARD_NO_PAD.encode(data);
    let mc = magic_crypt::new_magic_crypt!("scrumdiddlyumptious", 256);


    // let encrypted = mc.encrypt_str_to_base64(data);
    // println!("{}", encrypted);
    //
    // fs::write("/home/foxx/.sekret_enc", encrypted.as_bytes()).unwrap();

    let file_read = fs::read("/home/foxx/.sekret_enc").unwrap();
    let df: &str = std::str::from_utf8(file_read.as_slice()).unwrap();

    let decrypted = mc.decrypt_base64_to_string(df).unwrap();
    println!("{}", decrypted);
    //
    // let decrypted = mc.decrypt_base64_to_string(data.).unwrap();
    //
    // println!("{}", encoded);
    // let decrypted = mc.decrypt_base64_to_string(encoded).unwrap();
    //
    // println!("{:?}", decrypted);


    // let mut enigo = Enigo::new();
    // thread::sleep(Duration::new(1,0));
    // enigo.key_sequence(data.as_str());
}
