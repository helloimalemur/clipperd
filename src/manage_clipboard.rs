use std::{fs, process, thread};
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



struct Clippard {
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


impl Clipping for Clippard {
    fn clip_board_one() {
        // arc/mutex clipboard1 variable? second thread for write/listen hotkey?
        thread::spawn(|| {
            let mut keybind = Keybind::new(&[Keycode::LControl, Keycode::G]);
            keybind.on_trigger(|| {
                println!("{}", "keyprss");

                let mut clipboard = Clipboard::new().unwrap();
                println!("Clipboard text was: {}", clipboard.get_text().unwrap());
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


fn listen_for_clipboards() {
    <Clippard as Clipping>::clip_board_one();
    // <Clippard as Clipping>::clip_board_two();
    // <Clippard as Clipping>::clip_board_three();
    // <Clippard as Clipping>::clip_board_four();
    // <Clippard as Clipping>::clip_board_five();
}
