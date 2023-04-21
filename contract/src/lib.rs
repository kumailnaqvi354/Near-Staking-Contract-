use std::cmp::Ordering;
use std::collections::HashMap;

// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde_json::error;
use near_sdk::{env, log, near_bindgen, Promise};

// Define the default message
// const DEFAULT_MESSAGE: &str = "Hello";

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    balances: HashMap<String, u128>,
    stake_time: HashMap<String, u64>,
    total_staked: u128,
    token_id: String,
}

// Define the default, which automatically initializes the contract
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(token_id: String) -> Self {
        Self {
            balances: HashMap::new(),
            stake_time: HashMap::new(),
            total_staked: 0,
            token_id,
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    fn stake_tokens(mut self, user_wallet: String, amount: u128){
        let current_timestamp = env::block_timestamp();
        self.balances.insert(user_wallet.clone(), env::attached_deposit());
        self.stake_time.insert(user_wallet.clone(), current_timestamp);
        self.total_staked += amount;
        Promise::new(env::current_account_id()).transfer(amount);
        }

        fn calculate_reward(self, user_wallet: String) {
         let mut is_staked = self.balances.get(&user_wallet.clone());
         if is_staked <= Some(&0) {
            log!("No staking");   
         }
         
        }






    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    // pub fn get_greeting(&self) -> String {
    //     return self.message.clone();
    // }

    // Public method - accepts a greeting, such as "howdy", and records it
    // pub fn set_greeting(&mut self, message: String) {
    //     log!("Saving greeting {}", message);
    //     self.message = message;
    // }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn get_default_greeting() {
//         let contract = Contract::default();
//         // this test did not call set_greeting so should return the default "Hello" greeting
//         assert_eq!(contract.get_greeting(), "Hello".to_string());
//     }

//     #[test]
//     fn set_then_get_greeting() {
//         let mut contract = Contract::default();
//         contract.set_greeting("howdy".to_string());
//         assert_eq!(contract.get_greeting(), "howdy".to_string());
//     }
// }
