use std::{fs, process, thread};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::time::{Duration, SystemTime};
use arboard::Clipboard;
use enigo::{Enigo, KeyboardControllable};
use keybind::{Keybind, Keycode};
use magic_crypt::MagicCryptTrait;
use magic_crypt;


pub fn start_daemon() {
    let ostype = os_info::get().os_type().to_string();
    println!("{}", ostype);
    if ostype.contains("Linux") {
        // TODO: linux daemon
        println!("{}", "start linux daemon");
        start(true);
    } else {
        // TODO: windows service
        println!("{}", "start windows service");
        start(false);
    }
}

fn start (is_linux:bool) {
    listen_for_clipboards(is_linux); // start clipboard keybinding listeners
    loop {
        // listen for keystrokes
        thread::sleep(Duration::new(15,0));
    }
}

fn listen_for_clipboards(is_linux:bool) {

    clip_board_one_read(is_linux);
    clip_board_one_write(is_linux);

    clip_board_two_read(is_linux);
    clip_board_two_write(is_linux);

    clip_board_three_read(is_linux);
    clip_board_three_write(is_linux);

    clip_board_four_read(is_linux);
    clip_board_four_write(is_linux);

    clip_board_five_read(is_linux);
    clip_board_five_write(is_linux);

}


//////// one
fn clip_board_one_read(is_linux:bool) {
    thread::spawn(|| {
        println!("{}", "Thread 1, read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::F1]);
        keybind.on_trigger(|| {
            access_clip_board(1,false, is_linux);
        });
        keybind.wait();
    });
}

fn clip_board_one_write(is_linux:bool) {
    thread::spawn(|| {
        println!("{}", "Thread 1, write, started");
        let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F1]);
        keybind.on_trigger(|| {
            access_clip_board(1, true, is_linux);
        });
        keybind.wait();
    });
}

//////// two
fn clip_board_two_read(is_linux:bool) {
    thread::spawn(|| {
        println!("{}", "Thread 2, read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::F2]);
        keybind.on_trigger(|| {
            access_clip_board(2,false, is_linux);
        });
        keybind.wait();
    });
}

fn clip_board_two_write(is_linux:bool) {
    thread::spawn(|| {
        println!("{}", "Thread 2, write, started");
        let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F2]);
        keybind.on_trigger(|| {
            access_clip_board(2, true, is_linux);
        });
        keybind.wait();
    });
}

//////// three
fn clip_board_three_read(is_linux:bool) {
    thread::spawn(|| {
        println!("{}", "Thread 3, read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::F3]);
        keybind.on_trigger(|| {
            access_clip_board(3,false, is_linux);
        });
        keybind.wait();
    });
}

fn clip_board_three_write(is_linux:bool) {
    thread::spawn(|| {
        println!("{}", "Thread 3, write, started");
        let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F3]);
        keybind.on_trigger(|| {
            access_clip_board(3, true, is_linux);
        });
        keybind.wait();
    });
}

//////// four
fn clip_board_four_read(is_linux:bool) {
    thread::spawn(|| {
        println!("{}", "Thread 4, read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::F4]);
        keybind.on_trigger(|| {
            access_clip_board(4,false, is_linux);
        });
        keybind.wait();
    });
}

fn clip_board_four_write(is_linux:bool) {
    thread::spawn(|| {
        println!("{}", "Thread 4, write, started");
        let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F4]);
        keybind.on_trigger(|| {
            access_clip_board(4, true, is_linux);
        });
        keybind.wait();
    });
}

//////// five
fn clip_board_five_read(is_linux:bool) {
    thread::spawn(|| {
        println!("{}", "Thread 5, read, started");
        let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::F5]);
        keybind.on_trigger(|| {
            access_clip_board(5,false, is_linux);
        });
        keybind.wait();
    });
}

fn clip_board_five_write(is_linux:bool) {
    thread::spawn(|| {
        println!("{}", "Thread 5, write, started");
        let mut keybind = Keybind::new(&[Keycode::LShift, Keycode::F5]);
        keybind.on_trigger(|| {
            access_clip_board(5, true, is_linux);
        });
        keybind.wait();
    });
}


fn access_clip_board(selection: i32, save: bool, is_linux:bool) {
    let content = Clipboard::new().unwrap().get_text().expect("Could not retrieve clipboard");
    // encryption key
    let mc = magic_crypt::new_magic_crypt!("scrumdiddlyumptious", 256);
    let mut dest: String = String::new();
    // selection indicates which board
    println!("{}", dest);
    if is_linux {
        dest = format!("/var/lib/clippard/.board{}", selection);
        println!("{}", dest);
    } else {
        //     TODO:debug win
        dest = format!("C:/temp/.board{}", selection);
        println!("{}", dest);
    }

    // content is the string to save if writing
    // if save = true, write
    if save == true {
        fs::remove_file(Path::new(dest.clone().as_str())).unwrap_or_default();
        // content should be encrypted prior to writing
        let encrypted = mc.encrypt_str_to_base64(content);
        // // print encrypted and write to file



        let mut openfile = OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(dest)
            .expect("Create /var/lib/clippard");
        openfile.write(encrypted.as_bytes()).expect("Could not write file");
        // fs::write(dest, encrypted.as_bytes()).unwrap();
        openfile.sync_all().expect("cannot sync");
        println!("Wrote {}", selection);
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
        };

    }
}
