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
}