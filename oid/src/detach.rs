#![cfg(debug_assertions)]

use super::Cli;
use std::env::args;
use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::process::Command;

pub fn detach(cli: &Cli) -> Result<()> {
    let args: Vec<String> = args()
        .filter(|arg| arg != "--detach" && arg != "-d")
        .collect();

    let mut cmd = Command::new(args.first().unwrap());
    cmd.args(&args[1..]);

    if let Ok(child) = cmd.spawn() {
        if let Some(pid_path) = &cli.pid {
            fs::write(pid_path, format!("{}", child.id())).expect("Could not save PID into file");
        }
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Could not spawn a child"))
    }
}
