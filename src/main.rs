use std::process::Command;
use dialoguer::Select;

fn main() {
    let branches = Command::new("git")
        .arg("branch")
        .output()
        .unwrap()
        .stdout;

    let branches = String::from_utf8(branches).unwrap();
    let branches = parse_branches(branches);

    let choosen_branch = Select::new().items(&branches).interact().unwrap();

    let output = Command::new("git")
        .arg("checkout")
        .arg(&branches[choosen_branch])
        .output()
        .unwrap();

    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
}

fn parse_branches(branches: String) -> Vec<String> {
    branches
        .split("\n")
        .map(|el| el
            .trim()
            .trim_start_matches("* ")
            .to_string())
        .filter(|el| !el.is_empty())
        .collect()
}
