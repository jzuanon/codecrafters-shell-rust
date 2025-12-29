#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

    let builtins = ["echo", "exit", "type"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let parts: Vec<&str> = command.split_whitespace().collect();    
        match parts.get(0){
            Some(&"echo") => {
                println!("{}", &parts[1..].join(" "));
            },  
            Some(&"exit") => { 
                break; 
            },
            Some(&"type") => {
                if builtins.contains(&parts[1..].join(" ").as_str()) {
                    println!("{} is a shell builtin", &parts[1..].join(" "));
                }
                else {
                    println!("{}: command not found", &parts[1..].join(" "));
                }

            },
            _ => {
                println!("{}: command not found", command.trim());
            }
        }
        
    }
    

}
