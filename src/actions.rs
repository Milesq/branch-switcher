use dialoguer::{Select, MultiSelect};
use std::process::{Command, Output};

#[derive(Debug)]
pub enum ActionType {
    Checkout {
        previous: bool
    },
    Delete(bool),
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::Checkout { previous: false }
    }
}

static mut HARD_DELETE: bool = false;

pub fn get_action<'a>(action_type: ActionType) -> &'a dyn Fn(Vec<String>, usize) -> Vec<Output> {
    match action_type {
        ActionType::Checkout { previous } => {
            if previous {
                &previous
            } else {
                &checkout
            }
        },
        ActionType::Delete(hard) => {
            unsafe {
                HARD_DELETE = hard;
            }

            &delete
        },
    }
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

fn previous(branches: Vec<String>, current: usize) -> Vec<Output> {
    vec![]
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
                .arg(if unsafe { HARD_DELETE } { "-D" } else { "-d" })
                .arg(&branches[to_delete])
                .output()
                .unwrap()
        );
    }

    outputs
}
