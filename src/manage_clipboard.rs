use std::fs;
use magic_crypt::MagicCryptTrait;
use magic_crypt;

pub fn start_daemon(ostype: bool) {
    if ostype == false {
        // TODO: linux daemon
        start();
    } else {
        // TODO: windows service
        start();
    }
}

fn start () {
    loop {
        println!("{}", "loop started")
        // listen for keystrokes
    }
}

//
// struct Clipboard {
//     selection: i32,
//     content: String,
//     sekret: String,
//     base_dir: String,
// }
//
// trait Clipping {
//     fn set_clip_board_one() {}
//     fn set_clip_board_two() {}
//     fn set_clip_board_three() {}
//     fn set_clip_board_four() {}
//     fn set_clip_board_five() {}
// }
//
//
// impl Clipping for Clipboard {
//     fn set_clip_board_one() {
//         todo!()
//     }
//
//     fn set_clip_board_two() {
//         todo!()
//     }
//
//     fn set_clip_board_three() {
//         todo!()
//     }
//
//     fn set_clip_board_four() {
//         todo!()
//     }
//
//     fn set_clip_board_five() {
//         todo!()
//     }
// }
//
// fn set_clip_board(_selection: i32, content: String) {
//     let mc = magic_crypt::new_magic_crypt!("scrumdiddlyumptious", 256);
//
//     let data = match fs::read_to_string(content) {
//         Ok(x) => x,
//         Err(_) => "ERR".to_string(),
//     };
//
//
//
//     let encrypted = mc.encrypt_str_to_base64(data);
//     println!("{}", encrypted);
//
//     fs::write("/home/foxx/.sekret_enc", encrypted.as_bytes()).unwrap();
//
// }
//
//
// fn read_clip_board (_selection: i32) {
//     let mc = magic_crypt::new_magic_crypt!("scrumdiddlyumptious", 256);
//     let file_read = fs::read("/home/foxx/.sekret_enc").unwrap();
//
//
//     let df: &str = std::str::from_utf8(file_read.as_slice()).unwrap();
//     let decrypted = mc.decrypt_base64_to_string(df).unwrap();
//     println!("{}", decrypted);
//
//
//
//     // let mut enigo = Enigo::new();
//     // thread::sleep(Duration::new(1,0));
//     // enigo.key_sequence(data.as_str());
//
//
// }
