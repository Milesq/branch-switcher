use std::process::Command;

fn main() {
    let branches = Command::new("git")
        .arg("branch")
        .output()
        .unwrap()
        .stdout;

    let branches = String::from_utf8(branches).unwrap();
    let branches = parse_branches(branches);
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
