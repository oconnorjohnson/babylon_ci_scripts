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

    let package: String = Input::new()
        .with_prompt("Enter the npm package to add, with flags if necessary")
        .interact_text()
        .unwrap();

    for repo in repos { 
        println!("Processing repository: {}", repo);
        if let Err(e) = add_package(repo, &package) { 
            eprintln!("Failed to add package in repository: {}: {}", repo, e);
        }
    }
}

fn add_package(repo: &str, package: &str) -> Result<(), Box<dyn std::error::Error>> {
    let full_command = format!("pnpm add {}", package);

    Command::new("sh")
        .args(&["-c", &full_command])
        .current_dir(repo)
        .status()?;
    
        Ok(())
}