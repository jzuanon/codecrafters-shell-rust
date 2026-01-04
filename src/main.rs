use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};

use is_executable::IsExecutable;

fn main() {
    let builtins = ["echo", "exit", "type"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let parts: Vec<&str> = command.split_whitespace().collect();
        match parts.get(0) {
            Some(&"echo") => {
                println!("{}", &parts[1..].join(" "));
            }
            Some(&"exit") => {
                break;
            }
            Some(&"type") => {
                if builtins.contains(&parts[1..].join(" ").as_str()) {
                    println!("{} is a shell builtin", &parts[1..].join(" "));
                } else {
                    if let Some(path) = env::var_os("PATH") {
                        let mut found = false;
                        for dir in env::split_paths(&path) {
                            if dir
                                .join(&parts[1..].join(" "))
                                .with_extension("")
                                .with_extension("exe")
                                .is_file()
                            {
                                #[cfg(target_os = "windows")]
                                {
                                    if dir
                                        .join(&parts[1..].join(" "))
                                        .with_extension("exe")
                                        .as_path()
                                        .is_executable()
                                    {
                                        println!(
                                            "{} is {}", &parts[1..].join(" "),
                                            dir.join(&parts[1..].join(" "))
                                                .with_extension("exe")
                                                .display()
                                        );
                                        found = true;
                                        break;
                                    }
                                }

                                #[cfg(target_os = "linux")]
                                {
                                    if dir.join(&parts[1..].join(" ")).as_path().is_executable() {
                                        println!("{} is {}", &parts[1..].join(" "), dir.join(&parts[1..].join(" ")).display());
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }

                        if !found {
                            println!("{}: not found", &parts[1..].join(" "));
                        }
                    }
                }
            }
            _ => {
                println!("{}: command not found", command.trim());
            }
        }
    }
}
