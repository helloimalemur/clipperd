use std::{fs, process, thread};
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::sync::mpsc;
use arboard::Clipboard;
use enigo::{Enigo, KeyboardControllable};
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
    let mut instance = Clippard::new();
    // test
    // instance.board1 = "beans".to_string();
    // println!("{}", instance.board1);



    Clippard::clip_board_one_read(&mut instance.board1);
    Clippard::clip_board_one_write(&mut instance.board1);

    Clippard::clip_board_two_read(&mut instance.board2);
    Clippard::clip_board_three_write(&mut instance.board3);
    Clippard::clip_board_four_write(&mut instance.board4);
    Clippard::clip_board_five_read(&mut instance.board5);

}

pub struct Clippard {
    board1: String,
    board2: String,
    board3: String,
    board4: String,
    board5: String,
}

impl Clippard {
    pub fn new() -> Clippard {
        Clippard {
            board1: "".to_string(),
            board2: "".to_string(),
            board3: "".to_string(),
            board4: "".to_string(),
            board5: "".to_string(),
        }
    }
}

pub trait Clipping {
    fn clip_board_one_read(board:&mut String) {}
    fn clip_board_one_write(board: &mut String) {}
    fn clip_board_two_read(board:&mut String) {}
    fn clip_board_two_write(board:&mut String);
    fn clip_board_three_write(board:&mut String) {}
    fn clip_board_three_read(board:&mut String);
    fn clip_board_four_write(board:&mut String) {}
    fn clip_board_four_read(board:&mut String);
    fn clip_board_five_read(board:&mut String) {}
    fn clip_board_five_write(board:&mut String);
}

impl Clipping for Clippard {
    // seems stupid but keyBind is blocking and I can't get around using a thread PER fxx keybind..

    //////// one
    fn clip_board_one_read(board: &mut String) {
        thread::spawn(|| {

            println!("{}", "Thread 1, key1, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F1]);

            keybind.on_trigger(|| {
                access_clip_board(1,false);
            });

            keybind.wait();
        });
    }

    fn clip_board_one_write(board: &mut String) {
        thread::spawn(|| {

            println!("{}", "Thread 1, key1, started");
            let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F1]);
            keybind.on_trigger(|| {

                // let mut clipboard = Clipboard::new().unwrap();
                // println!("Clipboard 1 text was: {}", clipboard.get_text().unwrap());

                access_clip_board(1, true);
            });
            keybind.wait();
        });
    }

    //////// two
    fn clip_board_two_read(board:&mut String) {
        thread::spawn(|| {

            println!("{}", "Thread 2, key2, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F2]);
            keybind.on_trigger(|| {

                // let mut clipboard = Clipboard::new().unwrap();
                // println!("Clipboard 2 text was: {}", clipboard.get_text().unwrap());

                access_clip_board(2, false);
            });
            keybind.wait();
        });
    }

    fn clip_board_two_write(board:&mut String) {
        thread::spawn(|| {

            println!("{}", "Thread 2, key2, started");
            let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F2]);
            keybind.on_trigger(|| {

                // let mut clipboard = Clipboard::new().unwrap();
                // println!("Clipboard 2 text was: {}", clipboard.get_text().unwrap());

                access_clip_board(2, true);
            });
            keybind.wait();
        });
    }

    //////// three
    fn clip_board_three_write(board:&mut String) {
        let board3: Mutex<&str> = Mutex::from("");
        thread::spawn(|| {
            println!("{}", "Thread 3, key3, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F3]);
            keybind.on_trigger(|| {

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard 3 text was: {}", clipboard.get_text().unwrap());
                access_clip_board(3, false);
            });
            keybind.wait();
        });
    }

    fn clip_board_three_read(board:&mut String) {
        let board3: Mutex<&str> = Mutex::from("");
        thread::spawn(|| {
            println!("{}", "Thread 3, key3, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F3]);
            keybind.on_trigger(|| {

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard 3 text was: {}", clipboard.get_text().unwrap());
                access_clip_board(3, true);
            });
            keybind.wait();
        });
    }

    //////// four
    fn clip_board_four_write(board:&mut String) {
        let board4: Mutex<&str> = Mutex::from("");
        thread::spawn(|| {
            println!("{}", "Thread 4, key4, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F4]);
            keybind.on_trigger(|| {

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard 4 text was: {}", clipboard.get_text().unwrap());
                access_clip_board(4, false);
            });
            keybind.wait();
        });
    }

    fn clip_board_four_read(board:&mut String) {
        let board4: Mutex<&str> = Mutex::from("");
        thread::spawn(|| {
            println!("{}", "Thread 4, key4, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F4]);
            keybind.on_trigger(|| {

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard 4 text was: {}", clipboard.get_text().unwrap());
                access_clip_board(4, true);
            });
            keybind.wait();
        });
    }

    //////// five
    fn clip_board_five_read(board:&mut String) {
        // let mut board5: Arc<Mutex<String>> =Default::default();
        thread::spawn(|| {
            println!("{}", "Thread 5, key5, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F5]);
            keybind.on_trigger(|| {

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard 5 text was: {}", clipboard.get_text().unwrap());
                access_clip_board(5, false);

            });
            keybind.wait();
        });
    }

    fn clip_board_five_write(board:&mut String) {
        // let mut board5: Arc<Mutex<String>> =Default::default();
        thread::spawn(|| {
            println!("{}", "Thread 5, key5, started");
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::LShift, Keycode::F5]);
            keybind.on_trigger(|| {

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard 5 text was: {}", clipboard.get_text().unwrap());
                access_clip_board(5, true);

            });
            keybind.wait();
        });
    }
}


fn access_clip_board(selection: i32, save: bool) {
    let mut content = Clipboard::new().unwrap().get_text().unwrap();
    let mc = magic_crypt::new_magic_crypt!("scrumdiddlyumptious", 256);
    let mut dest: String = String::new();
    // selection indicates which board
    dest = format!("/tmp/board{}", selection);
    println!("{}", dest);

    // content is the string to save if writing
    // if save = true, write
    if save == true {
        // content should be encrypted prior to writing
        // TODO: WRITE
        // let path: &str = "/home/foxx/.sekret";
        // let data = match fs::read_to_string(path) {
        //     Ok(x) => x,
        //     Err(_) => "ERR".to_string(),
        // };
        let encrypted = mc.encrypt_str_to_base64(content);
        // // print encrypted and write to file
        println!("{}", encrypted);
        fs::write(dest, encrypted.as_bytes()).unwrap();
    } else {
        // if save = false, read
        // content should be output to keyboard if reading
        // TODO: READ
        // read encrypted
        let file_read = fs::read(dest).unwrap();
        //
        let df: &str = std::str::from_utf8(file_read.as_slice()).unwrap();
        let decrypted = mc.decrypt_base64_to_string(df).unwrap();
        println!("{}", decrypted);

        let mut enigo = Enigo::new();
        thread::sleep(Duration::new(1,0));
        enigo.key_sequence(decrypted.as_str());
    }



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
