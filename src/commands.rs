use crate:: ledger::Ledger;
use colored::Colorize;

pub fn handle_command(ledger: &mut Ledger, input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    match parts[0] {

        "help" => {
         println!("{}" , "Available commands:".yellow().bold());
         println!("{}", " create <name> <amount>            - Create a new account".cyan());
         println!("{}", "  balance <name>                    - Check account balance".cyan());
         println!("{}","  transfer <from> <to> <amount>     - Create Transaction".cyan());
         println!("{}","  list                              - List all accounts".cyan());
         println!("{}", "  save                              - Save content to json file".cyan());
         println!("{}","  quit / exit                       - Exit the program".cyan());
        }
        "create" => {
            if parts.len() != 3 {
                println!("{}", "Usage: create <name> <amount>".red());
                
            } 
            let name = parts[1].to_string();
            let amount:u64= match parts[2].parse() {
                Ok(data) => data,
                Err(_) => { println!("{}", "❌ Invalid, amount must be a positive number".red());
                return;
            }
            };

           match ledger.create_account(name.clone(), amount) {
        Ok(_) => println!("{} Account {} created with {} tokens", 
            "✅", 
            name.bright_cyan(), 
            amount.to_string().yellow()),
        Err(e) => println!("{} Error: {}", "❌".red(), e),
    }
        }

        "balance" => {
            if parts.len() != 2 {
                println!("{} You need to provide exisiting name, try:  <balance> <name>", "❌");
                return;
            }
            let name = parts[1];
            match ledger.get_balance(name) {
              Some(balance) => println!("{} {}", name, balance),
              None => println!("{} account {} not found!","❌", name.red())
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
            let accounts = ledger.list_accounts();

             if accounts.is_empty() {
            println!("{}", "No accounts created yet, Create one with: create <name> <amount>".yellow());
            return;
        }


            println!("\n{}", "╔══════════════════════════════════════╗".bright_blue());
            println!("{}", "║        ACCOUNT LEDGER                ║".bright_blue());
            println!("{}", "╠══════════════════════════════════════╣".bright_blue());
            
            let mut total_tokens = 0u64;

            for (name, balance) in &accounts {
                total_tokens += balance;
                    println!("║ {:<20} {:>8} {} ║", 
            name.bright_cyan(), 
            balance.to_string().yellow(),
            "tokens".white()
            );
         }

         println!("{}", "╠══════════════════════════════════════╣".bright_blue());
         println!("║ {:<20} {:>8} {} ║", 
            "TOTAL".bright_white().bold(), 
            total_tokens.to_string().green().bold(),
            "tokens".white()
    );
         println!("{}", "╚══════════════════════════════════════╝".bright_blue());
         println!();
        }

        "save" => {
            const SAVING_FILE: &str = "accounts.json"; 
            let manual_save = ledger.save_to_file(SAVING_FILE);
            
            match manual_save {
                Ok(_) =>{

                    println!("successfully saved file to {}", SAVING_FILE);
                } 
                Err(e) =>{

                    println!("Could not perform saving of file {}", e);
                } 
            }
        }

         _=> {
            println!("❌ Unknown command: '{}'. Type 'help' for available commands.", parts[0].red())
        }
 

    };


}