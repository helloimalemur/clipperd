use std::{fs, process, thread};
use std::time::Duration;
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

    loop {
        println!("{}", "loop started");
        // listen for keystrokes
        thread::sleep(Duration::new(3,0));
    }
}


struct Clipboard {
    selection: i32,
    content: String,
    sekret: String,
    base_dir: String,
}

trait Clipping {
    fn clip_board_one() {}
    fn clip_board_two() {}
    fn clip_board_three() {}
    fn clip_board_four() {}
    fn clip_board_five() {}
}


impl Clipping for Clipboard {
    fn clip_board_one() {
        thread::spawn(|| {
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::G]);
            keybind.on_trigger(|| {
                process::Command::new("firefox").spawn().unwrap();
            });
            keybind.wait();
        });
    }

    fn clip_board_two() {
        todo!()
    }

    fn clip_board_three() {
        todo!()
    }

    fn clip_board_four() {
        todo!()
    }

    fn clip_board_five() {
        todo!()
    }
}

fn set_clip_board(_selection: i32, content: String) {

}


fn read_clip_board () {



}
