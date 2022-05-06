use std::{
    fs,
    path::Path,
    process::{Command, Output},
};
use dialoguer::{MultiSelect, Select};
use crate::utils;

const PREVIOUS_BRANCH_FILENAME: &str = "./.git/previousBranch";

type ActionOut = Option<Vec<Output>>;

#[derive(Debug)]
pub enum ActionType {
    Checkout { previous: bool },
    Delete(bool),
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::Checkout { previous: false }
    }
}

static mut HARD_DELETE: bool = false;

pub fn get_action<'a>(action_type: ActionType) -> &'a dyn Fn(Vec<String>, usize) -> ActionOut {
    match action_type {
        ActionType::Checkout { previous } => {
            if previous {
                &previous_branch
            } else {
                &checkout
            }
        }
        ActionType::Delete(hard) => {
            unsafe {
                HARD_DELETE = hard;
            }

            &delete
        }
    }
}

fn checkout(branches: Vec<String>, current: usize) -> ActionOut {
    let choosen_branch = Select::new()
        .items(&branches)
        .default(current)
        .interact()
        .unwrap();

    Some(vec![utils::checkout(&branches[choosen_branch]).ok()?])
}

fn previous_branch(_: Vec<String>, _: usize) -> ActionOut {
    if !Path::new(PREVIOUS_BRANCH_FILENAME).exists() {
        eprintln!("It looks like you didn't switch the branch yet");
        return None;
    }

    let branch_name = fs::read_to_string(PREVIOUS_BRANCH_FILENAME).ok()?;

    Some(vec![utils::checkout(branch_name.trim()).ok()?])
}

fn delete(mut branches: Vec<String>, current: usize) -> ActionOut {
    branches.remove(current);

    let branches_to_delete = MultiSelect::new().items(&branches).interact().unwrap();

    let mut outputs = Vec::new();

    for to_delete in branches_to_delete {
        outputs.push(
            Command::new("git")
                .arg("branch")
                .arg(if unsafe { HARD_DELETE } { "-D" } else { "-d" })
                .arg(&branches[to_delete])
                .output()
                .ok()?,
        );
    }

    Some(outputs)
}
