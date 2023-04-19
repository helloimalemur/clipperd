use crate::manage_clipboard::start_daemon;
mod manage_clipboard;
extern crate daemonize;
use std::fs::File;
use daemonize::Daemonize;

fn main() {
    // TODO: debug daemon not writing out via onigo
    let stdout = File::create("/var/lib/clippard/daemon.out").unwrap();
    let stderr = File::create("/var/lib/clippard/daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/var/lib/clippard/clippard.pid") // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/var/lib/clippard/") // for default behaviour.
        .user("root")
        .group("root") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");
    match daemonize.start() {
        Ok(_) => println!("Success, daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }

    start_daemon();
}
