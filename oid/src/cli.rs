use clap::Parser;

#[cfg(debug_assertions)]
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Set to detach the daemon
    ///
    /// Debug mode only. Use with something like disown in release mode.
    #[cfg(debug_assertions)]
    #[arg(long, short)]
    pub detach: bool,

    /// Save the pid (process id) of the detached process (child)
    /// to the specified file
    ///
    /// Debug mode only.
    #[cfg(debug_assertions)]
    #[arg(long)]
    pub pid: Option<PathBuf>,

    // Specify port overriding the config value
    #[arg(long)]
    pub port: Option<u32>,

    /// Set amount of workers for the server to start with
    ///
    /// Set to 0 to let the app use as much workers as there are CPU cores
    #[arg(long, default_value = "1")]
    pub workers: usize,

    /// Print OpenApi3 json spec to the stdout and exit
    #[arg(long)]
    pub generate_openapi: bool,

    /// Specify amount of milliseconds which should be subtracted from all
    /// created timers, accounting for some known-in-advance
    /// lag
    ///
    /// Example: when new timer created for "1s" with latency of 10,
    /// actual timer will be fired for (1000 - 10 = 990)
    #[arg(long, default_value = "0")]
    pub latency: usize,
}
