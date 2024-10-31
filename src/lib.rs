use near_sdk::{env, require, log, near_bindgen, AccountId, BorshStorageKey, Promise, Gas};
use near_sdk::collections::UnorderedSet;

#[near_bindgen]
#[derive(Default)]
pub struct WhitelistContract {
    // A set of whitelisted addresses
    whitelist: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl WhitelistContract {
    #[init]
    pub fn new() -> Self {
        Self {
            whitelist: UnorderedSet::new(b"w".to_vec()),
        }
    }

    // Function to add an address to the whitelist
    pub fn add_to_whitelist(&mut self, account_id: AccountId) {
        self.whitelist.insert(&account_id);
        log!("Added {} to the whitelist", account_id);
    }

    // Function to remove an address from the whitelist
    pub fn remove_from_whitelist(&mut self, account_id: AccountId) {
        self.whitelist.remove(&account_id);
        log!("Removed {} from the whitelist", account_id);
    }

    // Function to check if an address is in the whitelist
    pub fn is_whitelisted(&self, account_id: &AccountId) -> bool {
        self.whitelist.contains(account_id)
    }

    // Main function to validate a transaction
    pub fn validate_transaction(&self, to: AccountId, amount: u128) -> bool {
        // Check if the recipient is whitelisted
        if self.is_whitelisted(&to) {
            log!("Transaction to whitelisted address {} is valid", to);
            return true;
        }

        // Assume that 95% threshold is 0.95 of the total transaction amount
        let threshold = (amount as f64 * 0.95) as u128;
        
        // Logic to ensure that the transaction doesn't send more than 95% to non-whitelisted addresses
        // This would require additional parameters (like change outputs) to check against
        // For simplicity, let's assume we just log and reject if it’s not going to whitelisted addresses
        log!("Transaction to non-whitelisted address {} exceeds 95% limit", to);
        false
    }

    // Example of a function that processes a transaction
    pub fn process_transaction(&self, to: AccountId, amount: u128) -> Promise {
        require!(self.validate_transaction(to.clone(), amount), "Transaction is invalid");

        // Here, you would normally proceed to process the transaction and call the necessary functions

        // Placeholder for transaction processing
        log!("Processing transaction of {} to {}", amount, to);

        Promise::new(env::current_account_id()) // Placeholder return value
    }
}
