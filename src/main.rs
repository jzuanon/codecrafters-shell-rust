#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

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
            _ => {
                println!("{}: command not found", command.trim());
            }
        }
        
    }
    

}
