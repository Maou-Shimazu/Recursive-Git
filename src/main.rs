use colored::Colorize;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let _args: Vec<String> = std::env::args().collect();

    if _args.len() == 1 {
        println!("Please use arguments `push` or `pull`.")
    } else {
        let base_path = Path::new(".");
        for entry in fs::read_dir(base_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                match _args[1].as_str() {
                    "pull" => {
                        std::env::set_current_dir(path.clone()).ok();
                        let cwd = std::env::current_dir()?;
                        if Path::new(".git/").exists() {
                            println!("\n{}", "Github Directory Detected.".green());
                            println!("{}: {}\n", "Pulling Directory".cyan(), cwd.display());

                            #[cfg(windows)]
                            Command::new("powershell")
                                .args(["/c", "git pull"])
                                .output()
                                .expect("Failed to git pull");

                            #[cfg(unix)]
                            Command::new("sh")
                                .args(["-c", "git pull"])
                                .output()
                                .expect("Failed to git pull");
                        } else {
                            println!("{1} {}", "is not a github directory.".red(), cwd.display());
                        }
                        std::env::set_current_dir("../").ok();
                    }
                    "push" => {
                        std::env::set_current_dir(path.clone()).ok();
                        let cwd = std::env::current_dir()?;
                        if Path::new(".git/").exists() {
                            println!("\n{}", "Github Directory Detected.".green());
                            println!("{}: {}\n", "Pulling Directory".cyan(), cwd.display());

                            #[cfg(windows)]
                            Command::new("powershell")
                                .args(["/c", "git commit -a -m \"Auto Committed by Recursive Git.\""])
                                .output()
                                .expect("Failed to git pull");

                            #[cfg(unix)]
                            Command::new("sh")
                                .args(["-c", "git commit -a -m \"Auto Committed by Recursive Git.\""])
                                .output()
                                .expect("Failed to git pull");
                        } else {
                            println!("{1} {}", "is not a github directory.".red(), cwd.display());
                        }
                        std::env::set_current_dir("../").ok();
                    }
                    _ => (),
                }
            }
        } //todo: check if user has wifi
    }
    Ok(())
}
