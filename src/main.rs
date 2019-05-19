use std::time::Duration;
use std::process::Command;
use futures::Future;
use tokio::timer::Timeout;
use tokio_process::CommandExt;
use std::io::{stdout, Write};

const TIMEOUT: u64 = 5;

fn main() {
    let unlocked_stdout = stdout();
    let mut stdout = unlocked_stdout.lock();
    let mut args = std::env::args_os();
    // 0th arg is path to self
    assert!(args.len() > 1, "Arguments needed");

    let command = args.next().unwrap();
    let output = Command::new(command).args(args).output_async();

    let future = output
        .map_err(|e| panic!("failed to collect output: {}", e))
        .map(|output| stdout.write(&output.stdout));

    let _ =  Timeout::new(future, Duration::from_secs(TIMEOUT));
}
