use std::{thread};
use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use thread::spawn;
use enigo::{Enigo, Key, KeyboardControllable};
use keybind::{Keybind, Keycode};
use magic_crypt::MagicCryptTrait;
use x11_clipboard::Clipboard;

pub fn clipperd() {
    let mut clipboard: Arc<Mutex<HashMap<u16, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    let cb1 = clipboard.clone();
    handles.push(spawn(move || {
        println!("{}", "Thread 1, write, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F1]);
        keybind.on_trigger(move || {
            println!("{}", "Thread 1, write, triggered");

            push_to_clipboard(1, "true", cb1.clone());
        });
        keybind.wait();
    }));

    let cb2 = clipboard.clone();
    handles.push(spawn(move || {
        println!("{}", "Thread 1, Read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::LAlt, Keycode::F1]);
        keybind.on_trigger(move || {
            println!("{}", "Thread 1, Read, triggered");

            get_from_clipboard(1, cb2.clone());
        });
        keybind.wait();
    }));

    let cb3 = clipboard.clone();
    handles.push(spawn(move || {
        println!("{}", "Thread 2, write, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F2]);
        keybind.on_trigger(move || {
            println!("{}", "Thread 2, write, triggered");

            push_to_clipboard(2, "true", cb3.clone());
        });
        keybind.wait();
    }));

    let cb4 = clipboard.clone();
    handles.push(spawn(move || {
        println!("{}", "Thread 2, Read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::LAlt, Keycode::F2]);
        keybind.on_trigger(move || {
            println!("{}", "Thread 2, Read, triggered");

            get_from_clipboard(2, cb4.clone());
        });
        keybind.wait();
    }));

    let cb5 = clipboard.clone();
    handles.push(spawn(move || {
        println!("{}", "Thread 3, write, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F3]);
        keybind.on_trigger(move || {
            println!("{}", "Thread 3, write, triggered");

            push_to_clipboard(3, "true", cb5.clone());
        });
        keybind.wait();
    }));

    let cb6 = clipboard.clone();
    handles.push(spawn(move || {
        println!("{}", "Thread 3, Read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::LAlt, Keycode::F3]);
        keybind.on_trigger(move || {
            println!("{}", "Thread 3, Read, triggered");

            get_from_clipboard(3, cb6.clone());
        });
        keybind.wait();
    }));


    let cb7 = clipboard.clone();
    handles.push(spawn(move || {
        println!("{}", "Thread 4, write, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F4]);
        keybind.on_trigger(move || {
            println!("{}", "Thread 4, write, triggered");

            push_to_clipboard(4, "true", cb5.clone());
        });
        keybind.wait();
    }));

    let cb8 = clipboard.clone();
    handles.push(spawn(move || {
        println!("{}", "Thread 4, Read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::LAlt, Keycode::F4]);
        keybind.on_trigger(move || {
            println!("{}", "Thread 4, Read, triggered");

            get_from_clipboard(4, cb6.clone());
        });
        keybind.wait();
    }));

    for e in handles {
        e.join().unwrap()
    }

}



fn push_to_clipboard(index: u16, string: &str, cb: Arc<Mutex<HashMap<u16, String>>>) {
    let mut clipboard_map = cb.lock().unwrap();

    let clipboard = Clipboard::new().unwrap();
    let primary = clipboard.load(
        clipboard.getter.atoms.primary,
        clipboard.getter.atoms.utf8_string,
        clipboard.getter.atoms.property,
        Duration::from_millis(100),
    ).unwrap();
    let content = String::from_utf8_lossy(&primary)
        .trim_matches('\u{0}')
        .trim()
        .to_string();
    println!("selection: {:#?}", content);
    // let content = Clipboard::new().unwrap().get_text().expect("Could not retrieve clipboard");

    // encryption key
    let mc = magic_crypt::new_magic_crypt!("scrumdiddlyumptious", 256);
    let mut dest: String = String::new();
    // selection indicates which board
    let encrypted = mc.encrypt_str_to_base64(content);
    // let df: &str = std::str::from_utf8(encrypted.as_bytes()).unwrap_or_default();
    // let decrypted = mc.decrypt_base64_to_string(df).unwrap_or_default();
    // let mut enigo = Enigo::new();
    // enigo.key_sequence(decrypted.as_str());

    let _ = clipboard_map.insert(index, encrypted);
    println!("{:#?}", clipboard_map);
}

fn get_from_clipboard(index: u16, arc: Arc<Mutex<HashMap<u16, String>>>) -> String {
    let mc = magic_crypt::new_magic_crypt!("scrumdiddlyumptious", 256);
    let cb = arc.lock().unwrap();
    let encrypted = cb.get(&index).unwrap();
    let df: &str = std::str::from_utf8(encrypted.as_bytes()).unwrap_or_default();
    let decrypted = mc.decrypt_base64_to_string(df).unwrap_or_default();
    let mut enigo = Enigo::new();
    thread::sleep(Duration::new(0, 500000000));
    decrypted.split('\n').for_each(|dec| {
        enigo.key_sequence(dec);
        enigo.key_click(Key::Return);
    });
    decrypted
}
