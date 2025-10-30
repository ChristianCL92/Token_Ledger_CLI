use std::collections::HashMap;

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

    pub fn list_accounts(&self){
        if self.accounts.is_empty() {
            println!("No accounts created yet, Create one with: create <name> <amount>");
            return;
        }

        for (key, value) in &self.accounts {
            println!("Registered accounts with name {} and amount {}", key, value);
        }

        println!();
    }
}