mod actions;
mod utils;

use std::{env, process::Command};

use actions::{get_action, ActionType};

fn main() {
    let (branches, current) = get_branches();

    let current_branch_idx = branches
        .iter()
        .position(|branch| branch == &current)
        .unwrap();

    let possible_args = [
        (vec!["-d", "--delete"], ActionType::Delete(false)),
        (vec!["-D"], ActionType::Delete(true)),
        (vec!["-"], ActionType::Checkout { previous: true }),
    ];

    let cli_args = env::args().skip(1).collect::<Vec<_>>();

    let mut action_type = Default::default();
    'args: for arg in possible_args {
        for variant in arg.0 {
            if cli_args.iter().any(|el| el == variant) {
                action_type = arg.1;
                break 'args;
            }
        }
    }

    let outputs = get_action(action_type)(branches, current_branch_idx);

    for output in outputs.unwrap() {
        println!("{}", String::from_utf8(output.stdout).unwrap());
        println!("{}", String::from_utf8(output.stderr).unwrap());
    }
}

fn get_branches() -> (Vec<String>, String) {
    let branches = Command::new("git").arg("branch").output().unwrap().stdout;

    let branches = String::from_utf8(branches).unwrap();

    parse_branches(branches)
}

fn parse_branches(branches: String) -> (Vec<String>, String) {
    let all_branches = branches
        .lines()
        .map(|el| el.trim())
        .filter(|el| !el.is_empty());

    let current_branch = all_branches
        .clone()
        .find(|branch| branch.starts_with("* "))
        .map(|el| el.trim_start_matches("* "))
        .unwrap()
        .to_string();

    let normalized_branch_names = all_branches.map(|el| el.trim_start_matches("* ").to_string());

    (normalized_branch_names.collect(), current_branch)
}
