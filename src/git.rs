use std::process::{Command, Output};

/// Executes the checkout of an existing branch.
pub(crate) fn checkout_branch(branch: &str) {
    let output: Output = Command::new("git")
        .args(["checkout", branch])
        .output()
        .expect("failed to execute checkout");
    print_result(output);
}

/// Executes the pulling of a branch.
pub(crate) fn pull(branch: &str) {
    let output: Output = Command::new("git")
        .args(["pull", "origin", branch])
        .output()
        .expect("failed to execute pull");
    print_result(output);
}

/// Executes the creation of a branch.
pub(crate) fn create_branch(branch: &str) {
    let output: Output = Command::new("git")
        .args(["checkout", "-b", branch, "--track", "origin/", branch])
        .output()
        .expect("failed to execute branch creation");
    print_result(output);
}

/// Prints the result of the command to stdout or panics if the command failed to ensure it doesn't continue.
fn print_result(output: Output) {
    if output.status.success() {
        println!("{}", String::from_utf8(output.stdout).unwrap());
    } else {
        panic!("{}", String::from_utf8(output.stderr).unwrap());
    }
}
