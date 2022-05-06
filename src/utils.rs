use std::{io, process::{Command, Output}};

pub fn checkout(branch: &str) -> io::Result<Output> {
    Command::new("git")
        .arg("checkout")
        .arg(branch)
        .output()
}
