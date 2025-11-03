use colored::Colorize;
mod ledger;
mod commands;
use ledger::Ledger;
use commands::handle_command;
use std::{io::{self, Write}};



fn main() {
    const SAVE_FILE: &str = "accounts.json";
  
    let mut ledger: Ledger = match Ledger::load_from_file(SAVE_FILE) {
        Ok(loaded) => {
            println!("{} Loading existing ledger from {}","✅".green(), SAVE_FILE.yellow());
            loaded
        }

        Err(_) => {
            println!("No existing file found");
            Ledger::new()
        }
    };
    
    println!("🪙 Token Ledger CLI");
    println!("Type 'help' for commands\n");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input  = String::new();
        io::stdin().read_line(&mut input).expect("Could not perform reading of input");
    
        let input: &str = input.trim();
        
        if input == "quit" || input == "exit" {
            println!("{}", "Saving ledger...".yellow());

        match ledger.save_to_file(SAVE_FILE) {
            Ok(_) => println!("✅ content saved to {}", SAVE_FILE),
            Err(e)  => println!("❌ Failed saving content to file {}", e)
        }    
            println!("Goodbye!");
            break;
        }
        
     handle_command(&mut ledger, input);
    }
}