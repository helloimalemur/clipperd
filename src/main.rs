// https://lib.rs/crates/arboard
// https://crates.io/crates/magic-crypt
// https://docs.rs/enigo/latest/enigo/
// https://github.com/enigo-rs/enigo
use enigo::*;
use std::{fs, thread};
use std::fs::File;
use std::time::Duration;
use base64::Engine;
use base64::engine::general_purpose;
use magic_crypt::MagicCryptTrait;
use daemonize;
use daemonize::{Daemonize, DaemonizeError};

mod manage_clipboard;


fn main() {
    let stdout = File::create("/var/log/clipperd.log").unwrap();
    let stderr = File::create("/var/log/clipperd_error.log").unwrap();

    let daemon = Daemonize::new()
        .pid_file("/tmp/clipperd.pid")
        .user("foxx")
        .group("foxx")
        .umask(0o777)
        .stdout(stdout)
        .stderr(stderr)
        .exit_action(|| println!("{}", "Exiting"))
        .privileged_action(|| println!("{}", "ran as priv"));

    match daemon.start() {
        Ok(_) => println!("{}", "Daemon running.."),
        Err(_) => eprintln!("{}", "Daemon could not be started")
    }
    println!("{}", "in daemon ..");
    thread::sleep(Duration::new(9,0));
    println!("{}", "leaving daemon ..");
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
