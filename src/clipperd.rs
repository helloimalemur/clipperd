use std::{fs, thread};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::Path;
use std::time::Duration;
use arboard::Clipboard;
use enigo::{Enigo, KeyboardControllable};
use keybind::{Keybind, Keycode};
use magic_crypt::MagicCryptTrait;

pub fn clipperd() {
    let mut clipboard: HashMap<String, String> = HashMap::new();




    let handle = thread::spawn(move || {
        println!("{}", "Thread 1, write, started");
        let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F1]);
        keybind.on_trigger(move || {
            println!("{}", "Thread 1, write, triggered");
            push_to_clipboard(1, "true", clipboard);
        });
        keybind.wait();
    });
    let _ = handle.join();


}



fn push_to_clipboard(index: i32, string: &str, clipboard: HashMap<String, String>) {
    let content = Clipboard::new().unwrap().get_text().expect("Could not retrieve clipboard");
    // encryption key
    let mc = magic_crypt::new_magic_crypt!("scrumdiddlyumptious", 256);
    let mut dest: String = String::new();
    // selection indicates which board
    let encrypted = mc.encrypt_str_to_base64(content);
    let df: &str = std::str::from_utf8(encrypted.as_bytes()).unwrap_or_default();
    let decrypted = mc.decrypt_base64_to_string(df).unwrap_or_default();
    let mut enigo = Enigo::new();
    enigo.key_sequence(decrypted.as_str());
}

fn get_from_clipboard(index: i32) -> String {
    // let df: &str = std::str::from_utf8(encrypted.as_bytes()).unwrap_or_default();
    // let decrypted = mc.decrypt_base64_to_string(df).unwrap_or_default();
    // let mut enigo = Enigo::new();
    // enigo.key_sequence(decrypted.as_str());
    String::new()
}
