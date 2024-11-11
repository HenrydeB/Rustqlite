use crate::vm::db::Database;
use std::io::{self, Write};
use std::collections::BTreeMap;
use std::fs::{OpenOptions, File};
use std::io::Read;
use std::thread;
use std::cell::RefCell;
use std::time::Duration;
use colored::*;

pub mod vm;
pub mod interpreter;

fn main(){
    println!("Starting RUSTQLITE...");

    print_title();    
    thread::sleep(Duration::from_secs(1));

    let db = match connect_db(){
        Ok(database) => database,
        Err(err) => {
            println!("{}", err);
            return;
        },
    };

    let db_cell = RefCell::new(db);
     
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
            let encode: Vec<u8> = bincode::serialize(&*db_cell.borrow()).unwrap();
            let mut file =  match File::create("data/database.rdb"){
                Ok(f) => f,
                Err(err) => {
                    println!("{}", err.to_string().red());
                    break;
                },
            };
                
            if let Err(err) = file.write_all(&encode){
                println!("{}", err.to_string().red());
                break;
            }
            println!("exiting...");
            break;
        }

        if trimmed == "schema"{
            if let Err(err) = print_schema(&db_cell.borrow()){
                println!("{}", err);
            }
            continue;
        }

        //this may not be the right way to go about this
        let statement = interpreter::interpret(&trimmed);

        match statement {
            Ok(stmt) => {
                match vm::process(stmt.clone(), db_cell.borrow_mut()){
                    Ok(msg) => println!("{}", msg),
                    Err(err) => println!("{}", err),
                }
            },
            Err(err) => println!("{}", err),
        }
    }
}


fn print_schema(database: &Database) -> Result<(), ColoredString>{ 

    let table_names: Vec<&String> = database.tables.keys().collect();

    for table in table_names{
        let name  = table;
        println!("{}", name.yellow());
    }

    Ok(())

}

fn connect_db() -> Result<Database, ColoredString>{
    
    let get_file = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .open("data/database.rdb");

    let mut file = match get_file{
        Ok(db) => db,
        Err(err) => return Err(err.to_string().red()),
    };

    let mut buff = Vec::new();

    match file.read_to_end(&mut buff){
        Ok(_) => {},
        Err(err) => return Err(err.to_string().red()), 
    } 
    
    let memory_db: Database = match bincode::deserialize(&buff){
        Ok(exists) => exists,
        Err(_) => {
            println!("{}", "No database found.. creating new DB instance".red());
            Database{
                tables : BTreeMap::new(),
            }
        },
    };
    Ok(memory_db)
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





