use crate::utils;
use dialoguer::{MultiSelect, Select};
use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

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

type ActionOut = Option<Vec<Output>>;

impl ActionType {
    pub fn dispatch(&self, branches: Vec<String>, current: usize) -> ActionOut {
        match self {
            ActionType::Checkout { previous } => {
                if *previous {
                    self.previous_branch(branches, current)
                } else {
                    self.checkout(branches, current)
                }
            }
            ActionType::Delete(_) => self.delete(branches, current),
        }
    }

    fn determine_previous_branch_filename() -> String {
        let mut previous_branch_path = utils::get_git_root().unwrap();

        previous_branch_path.push_str("./.git/previousBranch");

        previous_branch_path
    }

    fn checkout(&self, branches: Vec<String>, current: usize) -> ActionOut {
        let current_branch = branches[current].as_bytes();
        let choosen_branch = Select::new()
            .items(&branches)
            .default(current)
            .interact()
            .unwrap();

        let branch = branches[choosen_branch].as_str();

        let output = utils::checkout(branch);

        if output.is_ok() {
            fs::write(Self::determine_previous_branch_filename(), current_branch).unwrap();
        }

        Some(vec![output.ok()?])
    }

    fn previous_branch(&self, branches: Vec<String>, idx: usize) -> ActionOut {
        let previous_branch_filename = Self::determine_previous_branch_filename();
        let current_branch = branches[idx].as_bytes();

        if !Path::new(&previous_branch_filename).exists() {
            eprintln!("It looks like you didn't switch the branch yet");
            return None;
        }

        let branch_name_to_switch = fs::read_to_string(&previous_branch_filename).ok()?;

        fs::write(&previous_branch_filename, current_branch).unwrap();

        Some(vec![utils::checkout(branch_name_to_switch.trim()).ok()?])
    }

    fn delete(&self, mut branches: Vec<String>, current: usize) -> ActionOut {
        let hard_delete = if let ActionType::Delete(hard) = self {
            *hard
        } else {
            panic!("This should never happen");
        };

        branches.remove(current);

        let branches_to_delete = MultiSelect::new().items(&branches).interact().unwrap();

        let mut outputs = Vec::new();

        for to_delete in branches_to_delete {
            outputs.push(
                Command::new("git")
                    .arg("branch")
                    .arg(if hard_delete { "-D" } else { "-d" })
                    .arg(&branches[to_delete])
                    .output()
                    .ok()?,
            );
        }

        Some(outputs)
    }
}
