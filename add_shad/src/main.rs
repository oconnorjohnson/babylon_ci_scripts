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

    let component: String = Input::new()
        .with_prompt("Enter the shadcn-ui component to add")
        .interact_text()
        .unwrap();

    for repo in repos { 
        println!("Processing repository: {}", repo);
        if let Err(e) = add_component(repo, &component) { 
                eprintln!("Failed to add component in repository: {}: {}", repo, e);
        }
    }
}

fn add_component(repo: &str, component: &str) -> Result<(), Box<dyn std::error::Error>> { 
    let full_command = format!("pnpm dlx shadcn-ui@latest add {}", component);

    Command::new("sh")
        .args(&["-c", &full_command])
        .current_dir(repo)
        .status()?;
    
    Ok(())
}


