use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use dialoguer::Input;

fn main() -> io::Result<()> { 
    let directories = vec![
        "../../court",
        "../../da",
        "../../pd",
        "../../probation",
        "../../sheriff",
    ];

    let filename: String = Input::new()
        .with_prompt("Enter the filename to touch")
        .interact_text()?;
    for dir in &directories {
        let file_path = Path::new(dir).join(&filename);
        match File::create(&file_path) {
            Ok(mut file) => {
                    writeln!(file, "// This is a new file created by the add_file_to_all script.")?;
                    println!("Created file: {}", file_path.display());
            },
            Err(e) => eprintln!("failed to create file in {}: {}", dir, e), 
        }
    }

    println!("File '{}' has been added to all directories.", filename);
    Ok(())
}
