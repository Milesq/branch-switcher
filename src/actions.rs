use dialoguer::{Select, MultiSelect};
use std::process::{Command, Output};

pub fn get_action<'a>() -> &'a dyn Fn(Vec<String>, usize) -> Vec<Output> {
    &delete
}

fn checkout(branches: Vec<String>, current: usize) -> Vec<Output> {
    let choosen_branch = Select::new()
        .items(&branches)
        .default(current)
        .interact()
        .unwrap();

    vec![Command::new("git")
        .arg("checkout")
        .arg(&branches[choosen_branch])
        .output()
        .unwrap()]
}

fn delete(mut branches: Vec<String>, current: usize) -> Vec<Output> {
    branches.remove(current);

    let branches_to_delete = MultiSelect::new()
        .items(&branches)
        .interact()
        .unwrap();


    let mut outputs = Vec::new();

    for to_delete in branches_to_delete {
        outputs.push(
            Command::new("git")
                .arg("branch")
                .arg("-d")
                .arg(&branches[to_delete])
                .output()
                .unwrap()
        );
    }

    outputs
}
