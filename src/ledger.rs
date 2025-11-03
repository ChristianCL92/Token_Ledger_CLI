use std::collections::HashMap;
use std::io::Read;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Ledger {
    accounts: HashMap<String, u64>
    
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new()
        }
    }

    pub fn create_account (&mut self, name: String, initial_balance: u64) -> Result<(), String> {
        if self.accounts.contains_key(&name) {
            return Err(format!("Account '{}' already exists", name));
        }

        self.accounts.insert(name.clone(), initial_balance);
        Ok(())
    }

    pub fn get_balance (&self, name: &str) -> Option<u64> {
        self.accounts.get(name).copied()
    }

    pub fn transfer(&mut self, from: &str , to: &str,  amount: u64) -> Result<(), String> {
        if !self.accounts.contains_key(from) {
            return Err(format!("Could not find account {}", from))
        }

        if !self.accounts.contains_key(to) {
            return Err(format!("receiving account does not exist{}", to))
        }

      let sender_balance= self.accounts.get(from).expect("found no sender balance");

      if *sender_balance < amount {
        return Err(format!(
            "Insufficient funds. {} has {} tokens but tried to send {}",
            from, sender_balance, amount
        ));
    }

        *self.accounts.get_mut(from).unwrap() -= amount;
        *self.accounts.get_mut(to).unwrap() += amount;

    Ok(())
    
    }

    pub fn list_accounts(&self) -> Vec<(String, u64)> {
       
        let mut accounts: Vec<(String, u64)> = self.accounts.iter().map(|(name, balance)| (name.clone(), *balance)).collect();
        
        accounts.sort_by(|a, b|a.0.cmp(&b.0));

        accounts
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self)
        .map_err(|e| format!("Failed to serialize: {}", e))?;

        let mut file = File::create(filename).map_err(|e| format!("failed to create file {}", e))?;

        file.write_all(json.as_bytes()).map_err(|e| format!("Failed to write to file: {}",e))?;

        Ok(())
    }

     pub fn load_from_file(filename: &str ) -> Result<Self, String> {
        let mut file = File::open(filename).map_err(|e| format!("failed to open file{}", e))?;

        let mut content = String::new();
        file.read_to_string(& mut content).map_err(|e| format!("Could not read the file {}", e))?;

        let ledger_to_parse:Ledger = serde_json::from_str(&content)
        .map_err(|e| format!("Could not deserialize the file {}", e))?;
       
       Ok(ledger_to_parse)
    } 
}