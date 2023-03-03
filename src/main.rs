use crate::manage_clipboard::start_daemon;
mod manage_clipboard;

fn main() {
    // determine OS (os_info)
    let osinfo = os_info::get();
    println!("Type: {}", osinfo.os_type());
    println!("Version: {}", osinfo.version());
    println!("Bitness: {}", osinfo.bitness());
    let ostype = osinfo.os_type().clone().to_string().contains("indows");

    // Linux
    if ostype == false {
        // TODO:finish - linux
        // start_daemon(false);
        start_daemon();
    } else {
        // TODO:finish - windows
        start_daemon();
    }

    // start daemon/service based on host OS (daemonize?)
    // register hotkeys to grab from clipboard and save data encrypted (Enigo/magic_crypt)
    // registery hotkeys to retreive data and type as keyboard (Enigo)
}
