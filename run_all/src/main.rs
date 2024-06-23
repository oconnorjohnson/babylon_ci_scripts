use std::process::Command;
use std::path::Path;
use std::io::{self, Write};

fn main() { 
    // declare our project directories in a vector on the heap with initial values we use lowercase vec! macro
    let projects = vec![
        "../../court",
        "../../da",
        "../../pd",
        "../../probation",
        "../../sheriff",
    ];
   
    // use an iterator to `run dev` on each project dir in the vec 
    let children: Vec<_> = projects.iter().map(|project| {
        let project_dir = Path::new(project);

        Command::new("pnpm")
        .args(&["run", "dev"])
        .current_dir(&project_dir)
        .spawn()
        .expect(&format!("Failed to start dev server for {}", project))
    }).collect();

    println!("All dev servers started. Press enter to stop all servers.");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // kill processes on specific ports 
    let ports = vec![3000, 3001, 3002, 3003, 3004];
    for port in ports { 
        kill_process_on_port(port);
    }

    for (i, mut child) in children.into_iter().enumerate() { 
        let project_name = Path::new(&projects[i]).file_name().unwrap().to_str().unwrap();
        child.kill().expect(&format!("Failed to stop dev server for {}", project_name));
    }

    println!("All dev servers stopped.");
}

fn kill_process_on_port(port: u16) { 
    let output = Command::new("lsof")
        .args(&["-ti", &format!(":{}", port)])
        .output()
        .expect("Failed to execute lsof");
    
    let pid = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    if !pid.is_empty() {
        let status = Command::new("kill")
            .arg("-9")
            .arg(&pid)
            .status()
            .expect("Failed to execute kill command");
        
        if status.success() { 
           
        } else { 
            eprintln!("Failed to kill process {} on port {}", pid, port);
        }
    } else { 
        println!("no process found on port {}", port);
    }
}