use crate::manage_clipboard::start_daemon;
mod manage_clipboard;
extern crate daemonize;
use std::fs::File;
use daemonize::Daemonize;

fn main() {
    // let stdout = File::create("/tmp/daemon.out").unwrap();
    // let stderr = File::create("/tmp/daemon.err").unwrap();
    //
    // let daemonize = Daemonize::new()
    //     .pid_file("/tmp/test.pid") // Every method except `new` and `start`
    //     .chown_pid_file(true)      // is optional, see `Daemonize` documentation
    //     .working_directory("/tmp") // for default behaviour.
    //     .user("foxx")
    //     .group("foxx") // Group name
    //     .group(2)        // or group id.
    //     .umask(0o777)    // Set umask, `0o027` by default.
    //     .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
    //     .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
    //     .privileged_action(|| "Executed before drop privileges");
    // match daemonize.start() {
    //     Ok(_) => println!("Success, daemonized"),
    //     Err(e) => eprintln!("Error, {}", e),
    // }

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
}
