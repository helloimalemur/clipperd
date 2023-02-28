use daemonize;
use crate::manage_clipboard::start_daemon;
mod manage_clipboard;

fn main() {
    // determine OS (os_info)
    let osinfo = os_info::get();
    println!("Type: {}", osinfo.os_type());
    println!("Version: {}", osinfo.version());
    println!("Bitness: {}", osinfo.bitness());
    let ostype = osinfo.os_type().clone().to_string().contains("indows");
    if ostype {
        println!("{}", "OS is Windows");
        println!("{:?}", ostype);
    } else if let ostype = false {
        println!("{}", "OS is NOT Windows .. assuming Linux");
        println!("{:?}", ostype);
    }

    // Linux
    if ostype == false {
        // TODO:finish - linux
        start_daemon(false);
    } else {
        // TODO:finish - windows
        start_daemon(true);
    }

    // start daemon/service based on host OS (daemonize?)
    // register hotkeys to grab from clipboard and save data encrypted (Enigo/magic_crypt)
    // registery hotkeys to retreive data and type as keyboard (Enigo)

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
