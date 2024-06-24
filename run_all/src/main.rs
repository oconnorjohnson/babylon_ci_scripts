use std::process::Command;
use std::path::Path;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

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
    println!("Attempting to kill process on port {}", port);
    
    // First, attempt to close any browser connections
    close_browser_connections(port);
    
    // Wait a moment for connections to close
    thread::sleep(Duration::from_secs(1));
    
    // Now attempt to kill the process
    let lsof_output = Command::new("lsof")
        .args(&["-ti", &format!(":{}", port)])
        .output()
        .expect("Failed to execute lsof");

    if lsof_output.status.success() {
        let pids: Vec<String> = String::from_utf8_lossy(&lsof_output.stdout)
            .trim()
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        if !pids.is_empty() {
            for pid in pids {
                println!("Attempting to kill process with PID {} on port {}", pid, port);
                let kill_output = Command::new("kill")
                    .arg("-9")
                    .arg(&pid)
                    .output()
                    .expect("Failed to execute kill");

                if kill_output.status.success() {
                    println!("Successfully killed process (PID: {}) on port {}", pid, port);
                } else {
                    eprintln!("Failed to kill process (PID: {}) on port {}", pid, port);
                    println!("kill stderr: {}", String::from_utf8_lossy(&kill_output.stderr));
                }
            }
        } else {
            println!("No process found on port {} after closing browser connections", port);
        }
    } else {
        println!("lsof command failed for port {}", port);
    }
}

fn close_browser_connections(port: u16) {
    println!("Attempting to close browser connections on port {}", port);
    
    // Send a request to the server to trigger connection close
    let _ = Command::new("curl")
        .args(&["-s", &format!("http://localhost:{}", port)])
        .output();
    
    // For WebSocket connections, you might need to send a close frame
    // This is a simplified example and may need to be adjusted based on your specific setup
    let _ = Command::new("wscat")
        .args(&["-c", &format!("ws://localhost:{}", port), "--close"])
        .output();
}