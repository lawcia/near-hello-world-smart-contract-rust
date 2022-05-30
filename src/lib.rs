//! This contract is given a name then greets the user.
//!
//! [greet]: struct.HelloWorld.html#method.greet
//! [set_name]: struct.HelloWorld.html#method.set_name
//! [get_name]: struct.HelloWorld.html#method.get_name
//! [delete]: struct.HelloWorld.html#method.delete

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::LookupMap;

near_sdk::setup_alloc!();

// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct HelloWorld {
    // See more data types at https://doc.rust-lang.org/book/ch03-02-data-types.html
    names: LookupMap<String, String>
}

impl Default for HelloWorld {
    fn default() -> Self {
        Self {
            names: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl HelloWorld {
  
    pub fn get_name(&mut self) -> String {
        let account_id = env::signer_account_id();
        let mut option = self.names.get(&account_id);
        let name = option.get_or_insert_with(|| "".to_string());
        return name.to_string();
    }

    pub fn set_name(&mut self, name: String) {
        let account_id = env::signer_account_id();
        self.names.insert(&account_id, &name);
        let log_message = format!("The name has been set: {}", name);
        env::log(log_message.as_bytes());
    }

    pub fn delete(&mut self) {
        let name = String::from("");
        let account_id = env::signer_account_id();
        self.names.insert(&account_id, &name);
        env::log(b"Name has been removed");
    }
    
    pub fn greet(&mut self) -> String {
        let account_id = env::signer_account_id();
        let mut option = self.names.get(&account_id);
        let name = option.get_or_insert_with(|| "World".to_string());
        let greeting = format!("Hello {}!", name);
        return greeting;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn get_name() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let contract = HelloWorld::default();
        println!("Value of name: {:?}", contract.get_name());
        assert_eq!("", contract.get_name());
    }

     #[test]
    fn set_name() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = HelloWorld::default();
        contract.set_name("Amy".to_string());
        println!("Value set name: {:?}", contract.get_name());
        assert_eq!("Amy", contract.get_name());
    }

    #[test]
    fn set_name_then_delete() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = HelloWorld::default();
        contract.set_name("Amy".to_string());
        contract.delete();
        println!("Value after delete: {:?}", contract.get_name());
        assert_eq!("", contract.get_name());
    }

    #[test]
    fn greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = HelloWorld::default();
        contract.set_name("Bob".to_string());
        let greeting = contract.greet();
        println!("Value of greeting: {}", greeting);
        assert_eq!("Hello Bob!", greeting);
    }

    #[test]
    fn greeting_with_no_name() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = HelloWorld::default();
        let greeting = contract.greet();
        println!("Value of greeting: {}", greeting);
        assert_eq!("Hello World!", greeting);
    }
}