mod ledger;

use ledger::Ledger;
use std::{io::{self, Write}, u64};

fn handle_command(ledger: &mut Ledger, input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    match parts[0] {

        "help" => {
         println!("Available commands:");
         println!("  create <name> <amount>            - Create a new account");
         println!("  balance <name>                    - Check account balance");
         println!("  transfer <from> <to> <amount>     - Create Transaction");
         println!("  list                              - List all accounts");
         println!("  quit / exit                       - Exit the program");
        }
        "create" => {
            if parts.len() != 3 {
                println!("❌ Usage: create <name> <amount>");
                
            } 
            let name = parts[1].to_string();
            let amount:u64= match parts[2].parse() {
                Ok(data) => data, 
                Err(_) => { println!("❌ Invalid input, use a positive number");
                return;
            }
            };

            match ledger.create_account(name.clone(), amount) {
                Ok(data) => data,
                Err(e) => 
                    println!("❌ Error {}", e)
                
            }
        }

        "balance" => {
            if parts.len() != 2 {
                println!("You need to provide exisiting name, try:  <balance> <name>");
                return;
            }
            let name = parts[1];
            match ledger.get_balance(name) {
              Some(balance) => println!("{} {}", name, balance),
              None => println!("account {} not found!", name)
            }
        }


        "transfer" => {
            let from = parts[1];
            let to = parts[2];
            let amount:u64 = match parts[3].parse() {
                Ok(good) => good,
                Err(_) => {
                    println!("❌ Invalid amount. Please use a positive number.");
                    return;
        }
            };
            match ledger.transfer(from, to, amount) {
                Ok(_) => println!("✅ Transferred {} tokens from {} to {}", amount, from, to),
                Err(e) => println!("❌ Error: {}", e),
            }
        }

        "list" => {
            ledger.list_accounts();
        }

         _=> {
            println!("❌ Unknown command: '{}'. Type 'help' for available commands.", parts[0])
        }
 

    };


}

fn main() {
    let mut ledger: Ledger = Ledger::new();
    
    println!("🪙 Token Ledger CLI");
    println!("Type 'help' for commands\n");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        
        let mut input  = String::new();
        io::stdin().read_line(&mut input).expect("Could not perform reading of input");
    
        let input = input.trim();
        
        if input == "quit" || input == "exit" {
            println!("Goodbye!");
            break;
        }
        
     handle_command(&mut ledger, input);
    }
}