//! Example Contract
//!
//! A simple example Soroban contract demonstrating basic functionality.
//! This serves as a template for implementing the actual ArenaX contracts.

use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    /// Initialize the contract
    pub fn initialize(env: Env, admin: Address) {
        // Store admin address
        env.storage()
            .instance()
            .set(&String::from_str(&env, "admin"), &admin);
    }

    /// Get the admin address
    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&String::from_str(&env, "admin"))
            .unwrap()
    }

    /// Simple greeting function
    pub fn greet(env: Env, name: String) -> String {
        String::from_str(&env, &format!("Hello, {name}!"))
    }
}

#[cfg(test)]
mod test;
