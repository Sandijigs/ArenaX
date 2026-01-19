#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    /// Initialize the contract
    pub fn initialize(env: Env, admin: soroban_sdk::Address) {
        env.storage().instance().set(&symbol_short!("admin"), &admin);
        env.events().publish((symbol_short!("init"),), (admin,));
    }

    /// Get the admin address
    pub fn admin(env: Env) -> soroban_sdk::Address {
        env.storage().instance().get(&symbol_short!("admin")).unwrap()
    }

    /// Store a greeting message
    pub fn set_greeting(env: Env, user: soroban_sdk::Address, message: Symbol) {
        user.require_auth();
        env.storage().persistent().set(&user, &message);
        env.events().publish((symbol_short!("greet_set"),), (user, message));
    }

    /// Get a greeting message
    pub fn get_greeting(env: Env, user: soroban_sdk::Address) -> Symbol {
        env.storage().persistent().get(&user).unwrap_or(symbol_short!("hello"))
    }

    /// Get all stored greetings (limited example)
    pub fn get_all_greetings(env: Env) -> Vec<Symbol> {
        let mut greetings = vec![&env];
        // In a real implementation, you'd iterate through stored keys
        // This is just a demonstration
        greetings.push_back(symbol_short!("hello"));
        greetings
    }

    /// Simple counter example
    pub fn increment_counter(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&symbol_short!("counter")).unwrap_or(0);
        count += 1;
        env.storage().instance().set(&symbol_short!("counter"), &count);
        env.events().publish((symbol_short!("count"),), (count,));
        count
    }

    /// Get current counter value
    pub fn get_counter(env: Env) -> u32 {
        env.storage().instance().get(&symbol_short!("counter")).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};
    use soroban_sdk::{token, Address, Env};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let admin = Address::generate(&env);

        let contract = ExampleContract;
        contract.initialize(env.clone(), admin.clone());

        assert_eq!(contract.admin(env), admin);
    }

    #[test]
    fn test_greeting() {
        let env = Env::default();
        let user = Address::generate(&env);
        let message = symbol_short!("world");

        let contract = ExampleContract;
        contract.set_greeting(env.clone(), user.clone(), message.clone());

        assert_eq!(contract.get_greeting(env, user), message);
    }

    #[test]
    fn test_counter() {
        let env = Env::default();

        let contract = ExampleContract;

        assert_eq!(contract.increment_counter(env.clone()), 1);
        assert_eq!(contract.increment_counter(env.clone()), 2);
        assert_eq!(contract.get_counter(env), 2);
    }
}