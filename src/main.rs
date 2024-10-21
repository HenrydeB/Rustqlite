use std::io::{self, Write};
use std::thread;
use std::time::Duration;


pub mod interpreter;

fn main() {
    println!("Starting RUSTQLITE...");
    thread::sleep(Duration::from_secs(1)); 
    
    loop{
        
        print!("RQLITE > ");
        io::stdout().flush().unwrap();
        let mut cmd = String::new();

        if io::stdin().read_line(&mut cmd).is_err(){
            println!("Failed to read input");
            continue;
        }
        
        let trimmed = cmd.trim().to_lowercase();

        if trimmed == "exit"{
            println!("exiting...");
            break;
        }
        interpreter::interpret(&trimmed);

        //process commands
/*        println!("===========================================================");
        println!("===CName===CName===CName===CName===CName===CName===CName===");
        println!("|  CName | CName | CName | CName | CName | CName | CName  |");
        println!("|  CName | CName | CName | CName | CName | CName | CName  |");
        println!("|  CName | CName | CName | CName | CName | CName | CName  |");
        println!("|  CName | CName | CName | CName | CName | CName | CName  |");
        println!("===========================================================");*/
    }


}
