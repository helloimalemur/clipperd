use std::{fs, thread};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use arboard::Clipboard;
use enigo::{Enigo, KeyboardControllable};
use keybind::{Keybind, Keycode};
use magic_crypt::MagicCryptTrait;
use magic_crypt;


pub fn start_daemon() {
    start();
    // if ostype == false {
    //     // TODO: linux daemon
    //     println!("{}", "start linux daemon");
    //     start();
    // } else {
    //     // TODO: windows service
    //     println!("{}", "start windows service");
    //     start();
    // }
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

    clip_board_one_read();
    clip_board_one_write();

    clip_board_two_read();
    clip_board_two_write();

    clip_board_three_read();
    clip_board_three_write();

    clip_board_four_read();
    clip_board_four_write();

    clip_board_five_read();
    clip_board_five_write();

}


//////// one
fn clip_board_one_read() {
    thread::spawn(|| {

        println!("{}", "Thread 1, read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::F1]);

        keybind.on_trigger(|| {
            access_clip_board(1,false);
        });

        keybind.wait();
    });
}

fn clip_board_one_write() {
    thread::spawn(|| {

        println!("{}", "Thread 1, write, started");
        let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F1]);

        keybind.on_trigger(|| {
            access_clip_board(1, true);
        });

        keybind.wait();
    });
}

//////// two
fn clip_board_two_read() {
    thread::spawn(|| {

        println!("{}", "Thread 2, read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::F2]);

        keybind.on_trigger(|| {
            access_clip_board(2,false);
        });

        keybind.wait();
    });
}

fn clip_board_two_write() {
    thread::spawn(|| {

        println!("{}", "Thread 2, write, started");
        let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F2]);

        keybind.on_trigger(|| {
            access_clip_board(2, true);
        });

        keybind.wait();
    });
}

//////// three
fn clip_board_three_read() {
    thread::spawn(|| {

        println!("{}", "Thread 3, read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::F3]);

        keybind.on_trigger(|| {
            access_clip_board(3,false);
        });

        keybind.wait();
    });
}

fn clip_board_three_write() {
    thread::spawn(|| {

        println!("{}", "Thread 3, write, started");
        let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F3]);

        keybind.on_trigger(|| {
            access_clip_board(3, true);
        });

        keybind.wait();
    });
}

//////// four
fn clip_board_four_read() {
    thread::spawn(|| {

        println!("{}", "Thread 4, read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::F4]);

        keybind.on_trigger(|| {
            access_clip_board(4,false);
        });

        keybind.wait();
    });
}

fn clip_board_four_write() {
    thread::spawn(|| {

        println!("{}", "Thread 4, write, started");
        let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F4]);

        keybind.on_trigger(|| {
            access_clip_board(4, true);
        });

        keybind.wait();
    });
}

//////// five
fn clip_board_five_read() {
    thread::spawn(|| {

        println!("{}", "Thread 5, read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::F5]);

        keybind.on_trigger(|| {
            access_clip_board(5,false);
        });

        keybind.wait();
    });
}

fn clip_board_five_write() {
    thread::spawn(|| {

        println!("{}", "Thread 5, write, started");
        let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F5]);

        keybind.on_trigger(|| {
            access_clip_board(5, true);
        });

        keybind.wait();
    });
}


fn access_clip_board(selection: i32, save: bool) {
    let content = Clipboard::new().unwrap().get_text().expect("Could not retrieve clipboard");
    // encryption key
    let mc = magic_crypt::new_magic_crypt!("scrumdiddlyumptious", 256);
    let mut dest: String = String::new();
    // selection indicates which board
    println!("{}", dest);
    dest = format!("/tmp/board{}", selection);

    println!("{}", dest);

    // content is the string to save if writing
    // if save = true, write
    if save == true {
        fs::remove_file(Path::new(dest.clone().as_str())).unwrap_or_default();
        // content should be encrypted prior to writing
        let encrypted = mc.encrypt_str_to_base64(content);
        // // print encrypted and write to file

        println!("Wrote {}", selection);

        let mut openfile = OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(dest)
            .unwrap();
        openfile.write(encrypted.as_bytes()).expect("Could not write file");
        // fs::write(dest, encrypted.as_bytes()).unwrap();

    } else {
        // if save = false, read
        // content should be output to keyboard if reading
        // read encrypted
        let file_read = fs::read(dest.clone()).unwrap_or_default();
        if file_read.len() > 0 {
            let df: &str = std::str::from_utf8(file_read.as_slice()).unwrap_or_default();
            let decrypted = mc.decrypt_base64_to_string(df).unwrap_or_default();

            println!("printing {}", selection);

            let mut enigo = Enigo::new();
            thread::sleep(Duration::new(1,0));
            enigo.key_sequence(decrypted.as_str());
        }
    }
}
