use std::env::args_os;
use std::os::unix::process::ExitStatusExt;
use std::time::Duration;
use std::process::{Command,exit};
use tokio::timer::Timeout;
use tokio_process::CommandExt;
use futures::future::Future;

const TIMEOUT: u64 = 5;

fn main() {
    let default = "Failed to get panic message.";
    std::panic::set_hook(Box::new(move |p| {
        eprintln!("{}", p.payload().downcast_ref().unwrap_or(&default));
        std::process::exit(-1);
    }));

    // first arg is path to self
    let mut args = args_os().skip(1);
    let mut command = args.next().expect("Arguments needed");

    let limit = match command.as_os_str().to_str().map(str::parse) {
        Some(Ok(num)) => {
            command = args.next().expect("Command needed after timeout");
            num
        },
        _ => TIMEOUT
    };

    let child = Command::new(command).args(args).spawn_async().expect("Failed to spawn").map(|status|if !status.success() {
        match status.code() {
            Some(code) => exit(code),
            None       => panic!("Process terminated by signal {}",status.signal().unwrap())
        }
    });
    let timeout = Timeout::new(child, Duration::from_secs(limit)).map_err(|_|panic!("Command timed out"));
    tokio::run(timeout);
}
