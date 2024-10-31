use std::io::{self, Write};
use std::thread;
use std::time::Duration;
//use std::process;
use colored::*;
//use std::fs;

pub mod vm;
pub mod interpreter;

fn main() {
    println!("Starting RUSTQLITE...");

    thread::sleep(Duration::from_secs(1)); 
    print_title();    
    loop{
        
        print!("RQLITE > ");
        io::stdout().flush().unwrap();
        let mut cmd = String::new();

        if io::stdin().read_line(&mut cmd).is_err(){
            println!("{}", "Failed to read input".red());
            continue;
        }
        
        let trimmed = cmd.trim().to_lowercase();

        if trimmed == "exit"{
            println!("exiting...");
            break;
        }

        //this may not be the right way to go about this
        let statement = interpreter::interpret(&trimmed);

        match statement {
            Ok(stmt) => {
                match vm::process(stmt.clone()){
                    Ok(msg) => println!("{}", msg),
                    Err(err) => println!("{}", err),
                }
            },
            Err(err) => println!("{}", err),
        }
    }
}

fn print_title(){
    println!("{}", r"______          _   _____ _     _ _            /\ ".red());
    println!("{}",r"| ___ \        | | |  _  | |   (_) |          ( /   @ @    ()".red());
    println!("{}",r"| |_/ /   _ ___| |_| | | | |    _| |_ ___      \  __| |__  /".red());
    println!("{}", r"|    / | | / __| __| | | | |   | | __/ _ \      -/   V   \-".red());
    println!("{}",r"| |\ \ |_| \__ \ |_\ \/' / |___| | ||  __/     /-|       |-\".red());
    println!("{}",r"\_| \_\__,_|___/\__|\_/\_\_____/_|\__\___|    / /-\     /-\ \".red());
    println!("{}",r"                                              / /-\     /-\ \".red());
    println!("{}",r"                                               / /-`---'-\ \".red());
    println!("{}",r"                                                /         \".red());
}





