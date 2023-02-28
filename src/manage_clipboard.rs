use std::{fs, process, thread};
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use arboard::Clipboard;
use keybind::{Keybind, Keycode};
use magic_crypt::MagicCryptTrait;
use magic_crypt;


pub fn start_daemon(ostype: bool) {
    if ostype == false {
        // TODO: linux daemon
        println!("{}", "start linux daemon");
        start();
    } else {
        // TODO: windows service
        println!("{}", "start windows service");
        start();
    }
}

fn start () {
    listen_for_clipboards(); // start clipboard keybinding listeners

    loop {
        println!("{}", "loop started");
        // listen for keystrokes
        thread::sleep(Duration::new(15,0));
    }
}

fn listen_for_clipboards() {
    let instance = Clippard::new(&Clippard {
        selection: 0,
        content: "".to_string(),
        sekret: "".to_string(),
        base_dir: "".to_string(),
    });

    // <Clippard as Clipping>::clip_board_one();
    // <Clippard as Clipping>::clip_board_two();
    // <Clippard as Clipping>::clip_board_three();
    // <Clippard as Clipping>::clip_board_four();
    // <Clippard as Clipping>::clip_board_five();
}

pub struct Clippard {
    selection: i32,
    content: String,
    sekret: String,
    base_dir: String,
}

impl Clippard {
    fn new() -> Clippard {}
}

pub trait Clipping {
    fn clip_board_one() {}
    fn clip_board_two() {}
    fn clip_board_three() {}
    fn clip_board_four() {}
    fn clip_board_five() {}
}

impl Clipping for Clippard {
    // seems stupid but keyBind is blocking and I can't get around using a thread PER fxx keybind..
    fn clip_board_one() {

        // arc/mutex clipboard1 variable? second thread for write/listen hotkey?
        thread::spawn(|| {
            println!("{}", "Thread 1, key1, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F1]);
            keybind.on_trigger(|| {

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard 1 text was: {}", clipboard.get_text().unwrap());
                access_clip_board(1,clipboard.get_text().unwrap(), true);
            });
            keybind.wait();
        });
    }

    fn clip_board_two() {
        let board2: Mutex<&str> = Mutex::from("");

        thread::spawn(|| {
            println!("{}", "Thread 2, key2, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F2]);
            keybind.on_trigger(|| {

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard 2 text was: {}", clipboard.get_text().unwrap());
            });
            keybind.wait();
        });
    }

    fn clip_board_three() {
        let board3: Mutex<&str> = Mutex::from("");
        thread::spawn(|| {
            println!("{}", "Thread 3, key3, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F3]);
            keybind.on_trigger(|| {

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard 3 text was: {}", clipboard.get_text().unwrap());
            });
            keybind.wait();
        });
    }

    fn clip_board_four() {
        let board4: Mutex<&str> = Mutex::from("");
        thread::spawn(|| {
            println!("{}", "Thread 4, key4, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F4]);
            keybind.on_trigger(|| {

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard 4 text was: {}", clipboard.get_text().unwrap());
            });
            keybind.wait();
        });
    }

    fn clip_board_five() {
        // let mut board5: Arc<Mutex<String>> =Default::default();
        thread::spawn(|| {
            println!("{}", "Thread 5, key5, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F5]);
            keybind.on_trigger(|| {

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard 5 text was: {}", clipboard.get_text().unwrap());
                // let mut m = board5.lock().unwrap();
                // *m = clipboard.get_text().unwrap();
                // println!("{:?}", board5);

            });
            keybind.wait();
        });
    }
}


fn access_clip_board(selection: i32, content: String, save: bool) {

// do some encryption and keep it in memory, dont write this to disk
    // Example encryption and Enigo output to keyboard
    // encryption algo
    // let mc = magic_crypt::new_magic_crypt!("scrumdiddlyumptious", 256);

    // // read plain and encrypt
    // let path: &str = "/home/foxx/.sekret";
    // let data = match fs::read_to_string(path) {
    //     Ok(x) => x,
    //     Err(_) => "ERR".to_string(),
    // };
    // let encrypted = mc.encrypt_str_to_base64(data);
    // // print encrypted and write to file
    // println!("{}", encrypted);
    // fs::write("/home/foxx/.sekret_enc", encrypted.as_bytes()).unwrap();


    // read encrypted
    // let file_read = fs::read("/home/foxx/.sekret_enc").unwrap();
    //
    // let df: &str = std::str::from_utf8(file_read.as_slice()).unwrap();
    // let decrypted = mc.decrypt_base64_to_string(df).unwrap();
    // println!("{}", decrypted);

    // let mut enigo = Enigo::new();
    // thread::sleep(Duration::new(1,0));
    // enigo.key_sequence(data.as_str());


}
