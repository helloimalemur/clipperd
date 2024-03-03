use std::collections::HashMap;
use std::thread;

use enigo::{Enigo, Key, KeyboardControllable};
use keybind::{Keybind, Keycode};
use magic_crypt::MagicCryptTrait;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use thread::spawn;
use x11_clipboard::Clipboard;

pub fn clipperd() {
    let mut rand = rand::thread_rng();
    let random = rand.gen::<i128>();
    let encryption_key: Arc<Mutex<String>> = Arc::new(Mutex::new(random.to_string()));
    let clipboard: Arc<Mutex<HashMap<u16, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    let cb1 = clipboard.clone();
    let eb1 = encryption_key.clone();
    handles.push(spawn(move || {
        println!("Thread 1, write-out, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F1]);
        keybind.on_trigger(move || {
            println!("Thread 1, write-out, triggered");

            push_to_clipboard(1, &cb1, &eb1);
        });
        keybind.wait();
    }));

    let cb2 = clipboard.clone();
    let eb2 = encryption_key.clone();
    handles.push(spawn(move || {
        println!("Thread 1, Read, started");
        let mut keybind = Keybind::new(&[
            Keycode::LControl,
            Keycode::LShift,
            Keycode::LAlt,
            Keycode::F1,
        ]);
        keybind.on_trigger(move || {
            println!("Thread 1, Read, triggered");

            get_from_clipboard(1, &cb2, &eb2);
        });
        keybind.wait();
    }));

    let cb3 = clipboard.clone();
    let eb3 = encryption_key.clone();
    handles.push(spawn(move || {
        println!("Thread 2, write-out, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F2]);
        keybind.on_trigger(move || {
            println!("Thread 2, write-out, triggered");

            push_to_clipboard(2, &cb3, &eb3);
        });
        keybind.wait();
    }));

    let cb4 = clipboard.clone();
    let eb4 = encryption_key.clone();
    handles.push(spawn(move || {
        println!("Thread 2, Read, started");
        let mut keybind = Keybind::new(&[
            Keycode::LControl,
            Keycode::LShift,
            Keycode::LAlt,
            Keycode::F2,
        ]);
        keybind.on_trigger(move || {
            println!("Thread 2, Read, triggered");

            get_from_clipboard(2, &cb4, &eb4);
        });
        keybind.wait();
    }));

    let cb5 = clipboard.clone();
    let eb5 = encryption_key.clone();
    handles.push(spawn(move || {
        println!("Thread 3, write-out, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F3]);
        keybind.on_trigger(move || {
            println!("Thread 3, write-out, triggered");

            push_to_clipboard(3, &cb5, &eb5);
        });
        keybind.wait();
    }));

    let cb6 = clipboard.clone();
    let eb6 = encryption_key.clone();
    handles.push(spawn(move || {
        println!("Thread 3, Read, started");
        let mut keybind = Keybind::new(&[
            Keycode::LControl,
            Keycode::LShift,
            Keycode::LAlt,
            Keycode::F3,
        ]);
        keybind.on_trigger(move || {
            println!("Thread 3, Read, triggered");

            get_from_clipboard(3, &cb6, &eb6);
        });
        keybind.wait();
    }));

    let cb7 = clipboard.clone();
    let eb7 = encryption_key.clone();
    handles.push(spawn(move || {
        println!("Thread 4, write-out, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F4]);
        keybind.on_trigger(move || {
            println!("Thread 4, write-out, triggered");

            push_to_clipboard(4, &cb7, &eb7);
        });
        keybind.wait();
    }));

    let cb8 = clipboard.clone();
    let eb8 = encryption_key.clone();
    handles.push(spawn(move || {
        println!("Thread 4, Read, started");
        let mut keybind = Keybind::new(&[
            Keycode::LControl,
            Keycode::LShift,
            Keycode::LAlt,
            Keycode::F4,
        ]);
        keybind.on_trigger(move || {
            println!("Thread 4, Read, triggered");

            get_from_clipboard(4, &cb8, &eb8);
        });
        keybind.wait();
    }));

    for e in handles {
        match e.join() {
            Ok(x) => x,
            Err(_) => panic!("Could not re-join on all thread handles"),
        }
    }
}

fn push_to_clipboard(
    index: u16,
    cb: &Arc<Mutex<HashMap<u16, String>>>,
    enc_key: &Arc<Mutex<String>>,
) {
    let mut clipboard_map = match cb.lock() {
        Ok(x) => x,
        Err(_) => panic!("could not get lock on clipboard hashmap"),
    };
    let enc_key_bind = match enc_key.lock() {
        Ok(x) => x,
        Err(_) => panic!("could not get lock on encryption key"),
    };
    let clipboard = match Clipboard::new() {
        Ok(x) => x,
        Err(_) => panic!("could not produce instance of clipboard object"),
    };
    let primary = clipboard
        .load(
            clipboard.getter.atoms.primary,
            clipboard.getter.atoms.utf8_string,
            clipboard.getter.atoms.property,
            Duration::from_millis(100),
        )
        .unwrap_or_else(|_| "".as_bytes().to_vec());
    let content = String::from_utf8_lossy(&primary)
        .trim_matches('\u{0}')
        .trim()
        .to_string();
    // println!("selection: {:#?}", content);

    // encryption key
    let mc = magic_crypt::new_magic_crypt!(enc_key_bind.to_string(), 256);
    // selection indicates which board
    let encrypted = mc.encrypt_str_to_base64(content);
    let _ = clipboard_map.insert(index, encrypted);
    println!("{:#?}", clipboard_map);
}

fn get_from_clipboard(
    index: u16,
    clipboard: &Arc<Mutex<HashMap<u16, String>>>,
    enc_key: &Arc<Mutex<String>>,
) -> String {
    let enc_key_bind = match enc_key.lock() {
        Ok(x) => x,
        Err(_) => panic!("could not get lock on clipboard hashmap"),
    };
    let mc = magic_crypt::new_magic_crypt!(enc_key_bind.to_string(), 256);
    let cb = match clipboard.lock() {
        Ok(x) => x,
        Err(_) => panic!("could not get lock on clipboard"),
    };
    let mut decrypted = String::new();
    let encrypted = match cb.get(&index) {
        Some(x) => x,
        None => "",
    };
    if !encrypted.is_empty() {
        let df: &str = std::str::from_utf8(encrypted.as_bytes()).unwrap_or_default();
        decrypted = mc.decrypt_base64_to_string(df).unwrap_or_default();
        let mut enigo = Enigo::new();
        thread::sleep(Duration::new(0, 500000000));
        let mut first: bool = true;
        decrypted.split('\n').for_each(|dec| {
            if !first {
                enigo.key_click(Key::Return);
            }
            first = false;
            enigo.key_sequence(dec);
        });
    }
    decrypted
}
