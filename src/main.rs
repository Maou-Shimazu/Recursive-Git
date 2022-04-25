use std::fs;
use std::process::Command;
use colored::Colorize;

fn main() -> std::io::Result<()> {
    let dir = std::env::current_dir()?;
    println!("{}: {}", "Current directory".cyan(), dir.display());
    let paths = fs::read_dir(dir).unwrap();
    let mut vec: Vec<String> = Vec::new();

    for path in paths {
        vec.push(path.unwrap().path().to_str().unwrap().to_owned());
    }

    for i in vec {
        if i.as_str().contains("\\") && !(i.as_str().contains(".")) {
                println!("{} {}", "Pulling".green(), i);
                let _cd = Command::new(format!("cd {}", i).as_str()).output();
                let _pull = Command::new("git pull").output();
        }
        else { println!("{}: {}", "Private folder or File".yellow(), i); }

    }
    Ok(())
 }
