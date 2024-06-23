use std::process::Command;
use dialoguer::Input;

fn main() { 
    let repos = vec![
        "../../court",
        "../../da",
        "../../pd",
        "../../probation",
        "../../sheriff",
    ];

    let commit_message: String = Input::new()
    .with_prompt("Enter commit message")
    .interact_text()
    .unwrap();

    for repo in repos {
        println!("Processing repository: {}", repo);
        if let Err(e) = process_repo(repo, &commit_message) {
            eprintln!("Failed to process repository: {}: {}", repo, e);
        }
    }
}

fn process_repo(repo: &str, commit_message: &str) -> Result<(), Box<dyn std::error::Error>> { 
    Command::new("git")
        .args(&["-C", repo, "add", "-A"])
        .status()?;
    Command::new("git")
        .args(&["-C", repo, "commit", "-m", commit_message])
        .status()?;
    Command::new("git")
        .args(&["-C", repo, "push"])
        .status()?;
    Ok(())
}
