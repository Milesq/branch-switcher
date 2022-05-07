use std::{
    io,
    process::{Command, Output},
};

pub fn checkout(branch: &str) -> io::Result<Output> {
    Command::new("git").arg("checkout").arg(branch).output()
}

pub fn get_git_root() -> Option<String> {
    String::from_utf8(
        Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()
            .ok()?
            .stdout,
    )
    .ok()
}
