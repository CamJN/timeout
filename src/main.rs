use futures::future::FutureExt;
use std::env::args_os;
use std::os::unix::process::ExitStatusExt;
use std::process::exit;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

const TIMEOUT: u64 = 5;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let default = "Failed to get panic message.";
    std::panic::set_hook(Box::new(move |p| {
        eprintln!("{}", p.payload().downcast_ref().unwrap_or(&default));
        std::process::exit(-1);
    }));

    // first arg is path to self
    let mut args = args_os().skip(1);
    let mut command = args.next().expect("Timeout needed");

    let limit = match command.to_str().map(str::parse) {
        Some(Ok(num)) => {
            command = args.next().expect("Command needed after timeout");
            num
        }
        _ => TIMEOUT,
    };

    let mut child = Command::new(command)
        .args(args)
        .spawn()
        .expect("Failed to spawn");

        let future = child.wait()
        .map(|rstatus| {
            if let Ok(status) = rstatus {
                match status.code() {
                    Some(code) => exit(code),
                    None => panic!("Process terminated by signal {}", status.signal().unwrap()),
                }
            }
        });

    let timeout = timeout(Duration::from_secs(limit), future);

    if let Err(_) = timeout.await {
        panic!("Command timed out");
    }
}
