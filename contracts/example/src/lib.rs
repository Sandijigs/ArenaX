#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec};

#[contract]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    /// Initialize the contract
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&Symbol::new(&env, "admin")) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.events().publish((Symbol::new(&env, "initialized"),), admin);
    }

    /// Get the admin address
    pub fn admin(env: Env) -> Address {
        env.storage().instance().get(&Symbol::new(&env, "admin")).unwrap()
    }

    /// Store a greeting message
    pub fn set_greeting(env: Env, user: Address, message: Symbol) {
        user.require_auth();
        env.storage().persistent().set(&user, &message);
        env.events().publish((Symbol::new(&env, "greeting_set"),), (user, message));
    }

    /// Get a greeting message
    pub fn get_greeting(env: Env, user: Address) -> Symbol {
        env.storage().persistent().get(&user).unwrap_or(Symbol::new(&env, "Hello!"))
    }

    /// Add a number to the counter
    pub fn increment_counter(env: Env, user: Address, amount: u32) {
        user.require_auth();

        let key = Symbol::new(&env, "counter");
        let current: u32 = env.storage().persistent().get(&key).unwrap_or(0);
        let new_value = current + amount;

        env.storage().persistent().set(&key, &new_value);
        env.events().publish((Symbol::new(&env, "counter_incremented"),), (user, amount, new_value));
    }

    /// Get the current counter value
    pub fn get_counter(env: Env) -> u32 {
        let key = Symbol::new(&env, "counter");
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Get contract version
    pub fn version(env: Env) -> Symbol {
        Symbol::new(&env, "1.0.0")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Env as _};
    use soroban_sdk::{Env, Address, Symbol};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ExampleContract);
        let client = ExampleContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        assert_eq!(client.admin(), admin);
    }

    #[test]
    fn test_greeting() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ExampleContract);
        let client = ExampleContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        let greeting = Symbol::new(&env, "Welcome to ArenaX!");

        client.set_greeting(&user, &greeting);
        assert_eq!(client.get_greeting(&user), greeting);
    }

    #[test]
    fn test_counter() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ExampleContract);
        let client = ExampleContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);

        assert_eq!(client.get_counter(), 0);

        client.increment_counter(&user, &5);
        assert_eq!(client.get_counter(), 5);

        client.increment_counter(&user, &3);
        assert_eq!(client.get_counter(), 8);
    }
}