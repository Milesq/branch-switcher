mod actions;

use std::process::Command;

use actions::get_action;

fn main() {
    let (branches, current) = get_branches();

    let current_branch_idx = branches.iter()
        .position(|branch| branch == &current)
        .unwrap();

    let output = get_action()(branches, current_branch_idx);
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
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
