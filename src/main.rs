#![allow(unused)]

pub mod commands;
pub mod git;

use clap::Parser;
use commands::{Arguments, Fallback, NewBranch, SubCommandExecutor};

fn main() {
    let args = Arguments::parse();
    let subcommand = args.subcommand;
    match subcommand.as_str() {
        "newb" => NewBranch::parse().run(),
        // We could panic here, but as a wrapper, it's just best to pass through the arguments to a good ol' Git command and close the loop.
        _ => {
            let fb = Fallback::parse();
            fb.run();
        }
    };
}
