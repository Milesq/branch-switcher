use dialoguer::Select;
use std::process::{Command, Output};

pub fn get_action<'a>() -> &'a dyn Fn(Vec<String>, usize) -> Output {
    &checkout
}

fn checkout(branches: Vec<String>, current: usize) -> Output {
    let choosen_branch = Select::new()
        .items(&branches)
        .default(current)
        .interact()
        .unwrap();

    Command::new("git")
        .arg("checkout")
        .arg(&branches[choosen_branch])
        .output()
        .unwrap()
}

fn delete(branches: Vec<String>, current: usize) -> Output {
    let choosen_branch = Select::new()
        .items(&branches)
        .default(current)
        .interact()
        .unwrap();

    Command::new("git")
        .arg("checkout")
        .arg(&branches[choosen_branch])
        .output()
        .unwrap()
}
