use std::fs;
use std::io;

fn main() {
    match fs::read_dir("C:\\") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        println!("{}", path.display());
                    }
                    Err(e) => println!("Error reading directory entry: {}", e),
                }
            }
        }
        Err(e) => println!("Error reading directory: {}", e),
    }
}