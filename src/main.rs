// Macros should be declared first
pub mod macros;

pub mod accessible;
pub mod archive;
pub mod cli;
pub mod commands;
pub mod error;
pub mod extension;
pub mod list;
pub mod progress;
pub mod utils;

/// CLI argparsing definitions, using `clap`.
pub mod opts;

use error::{Error, Result};
use opts::{Opts, Subcommand};
use utils::{QuestionAction, QuestionPolicy};

// Used in BufReader and BufWriter to perform less syscalls
const BUFFER_CAPACITY: usize = 1024 * 32;

/// The status code returned from `ouch` on error
pub const EXIT_FAILURE: i32 = libc::EXIT_FAILURE;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(EXIT_FAILURE);
    }
}

fn run() -> Result<()> {
    let (args, skip_questions_positively, file_visibility_policy) = Opts::parse_args()?;
    commands::run(args, skip_questions_positively, file_visibility_policy)
}
