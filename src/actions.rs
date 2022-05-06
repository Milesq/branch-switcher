use crate::utils;
use dialoguer::{MultiSelect, Select};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

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
    let current_branch = branches[current].as_bytes();
    let choosen_branch = Select::new()
        .items(&branches)
        .default(current)
        .interact()
        .unwrap();

    let branch = branches[choosen_branch].as_str();

    let output = utils::checkout(branch);

    if output.is_ok() {
        fs::write(PREVIOUS_BRANCH_FILENAME, current_branch).unwrap();
    }

    Some(vec![output.ok()?])
}

fn previous_branch(branches: Vec<String>, idx: usize) -> ActionOut {
    let current_branch = branches[idx].as_bytes();
    if !Path::new(PREVIOUS_BRANCH_FILENAME).exists() {
        eprintln!("It looks like you didn't switch the branch yet");
        return None;
    }

    let branch_name_to_switch = fs::read_to_string(PREVIOUS_BRANCH_FILENAME).ok()?;

    fs::write(PREVIOUS_BRANCH_FILENAME, current_branch).unwrap();

    Some(vec![utils::checkout(branch_name_to_switch.trim()).ok()?])
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
