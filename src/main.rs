use std::fs;
use std::process::Command;
use colored::Colorize;

fn main() -> std::io::Result<()> {
    let mut dir = std::env::current_dir()?;
    println!("{}: {}", "Current directory".cyan(), dir.display());
    let paths = fs::read_dir(dir.clone()).unwrap();
    let mut vec: Vec<String> = Vec::new();

    for path in paths {
        vec.push(path.unwrap().path().to_str().unwrap().to_owned());
    }
    dir.push(".git");
    let _git = std::fs::metadata(dir)?;
    for i in vec {
        if i.as_str().contains("\\") && !(i.as_str().contains(".")) {
                std::env::set_current_dir(i.clone()).ok();

            if _git.is_dir() {
            println!("{}", "Github Directory Detected.".green());
            println!("{} {}", "Pulling".green(), i);

                #[cfg(windows)]
                Command::new("powershell").args(["/c", "git pull"]).output().expect("Failed to git pull");

                #[cfg(unix)]
                Command::new("sh").args(["-c", "git pull"]).output().expect("Failed to git pull");
            }

                //format!("cd {}", i).as_str()).output();
        }
        else { println!("{}: {}", "Private folder or File".yellow(), i); }

    }
    Ok(())
 }
