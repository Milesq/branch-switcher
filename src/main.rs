use std::process::Command;
use dialoguer::Select;

fn main() {
    let branches = Command::new("git")
        .arg("branch")
        .output()
        .unwrap()
        .stdout;

    let branches = String::from_utf8(branches).unwrap();
    let (branches, current) = parse_branches(branches);
    let current = branches.iter()
        .position(|branch| branch == &current)
        .unwrap();

    let choosen_branch = Select::new()
        .items(&branches)
        .default(current)
        .interact()
        .unwrap();

    let output = Command::new("git")
        .arg("checkout")
        .arg(&branches[choosen_branch])
        .output()
        .unwrap();

    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
}

fn parse_branches(branches: String) -> (Vec<String>, String) {
    let all_branches = branches
        .split('\n')
        .map(|el| el
            .trim())
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
