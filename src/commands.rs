use std::process::{Command, Output};

use clap::Parser;

use crate::git;

/// Trait for subcommands to be executed.
pub(crate) trait SubCommandExecutor {
    /// Implement this function to run the subcommand by actually running a sequence of underlying commands.
    fn run(&self);
}

/// Struct for the checking out new branch subcommand.
#[derive(Parser)]
pub(crate) struct NewBranch {
    /// The name of the branch to checkout create from. Usually master or main.
    pub(crate) from_branch: String,
    /// The name of the branch to create.
    pub(crate) to_branch: String,
}

/// Struct for the fallback. Basically used when no command was found.
#[derive(Parser)]
pub(crate) struct Fallback {
    /// Default arguments of this default implementation.
    pub(crate) arguments: Vec<String>,
}

/// Implementation of the SubCommandExecutor trait for the Fallback struct.
/// Redirects the arguments to the git command.
impl SubCommandExecutor for Fallback {
    fn run(&self) {
        let output: Output = Command::new("git")
            .args(&self.arguments)
            .output()
            .expect("failed to execute git command");

        if output.status.success() {
            let result = String::from_utf8(output.stdout).unwrap();
            println!("{}", result);
        } else {
            panic!("{}", String::from_utf8(output.stderr).unwrap());
        }
    }
}

/// Main Arguments struct. Used to parse the arguments of the program.
#[derive(Parser)]
pub(crate) struct Arguments {
    /// The subcommand to run
    pub(crate) subcommand: String,
    /// The array of parameters to pass to the subcommand
    pub(crate) arguments: Vec<String>,
}

/// Implementation of the SubCommandExecutor trait for the CheckoutNewBranch struct.
impl SubCommandExecutor for NewBranch {
    fn run(&self) {
        git::checkout_branch(&self.from_branch);
        git::pull(&self.from_branch);
        git::create_branch(&self.to_branch);
    }
}
