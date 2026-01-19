use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec, Map};

use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};

#[contract]
pub struct ArenaxToken;

#[contractimpl]
impl ArenaxToken {
    /// Initialize the token contract
    pub fn initialize(env: Env, admin: Address, name: Symbol, symbol: Symbol, decimals: u32, initial_supply: i128) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &decimals);
        env.storage().instance().set(&DataKey::TotalSupply, &initial_supply);

        // Mint initial supply to admin
        env.storage().persistent().set(&DataKey::Balance(admin.clone()), &initial_supply);
        env.storage().persistent().bump(&DataKey::Balance(admin), BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);

        env.events().publish((Symbol::new(&env, "initialize"),), (admin, name, symbol, decimals, initial_supply));
    }

    /// Get token name
    pub fn name(env: Env) -> Symbol {
        env.storage().instance().get(&DataKey::Name).unwrap()
    }

    /// Get token symbol
    pub fn symbol(env: Env) -> Symbol {
        env.storage().instance().get(&DataKey::Symbol).unwrap()
    }

    /// Get token decimals
    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Decimals).unwrap()
    }

    /// Get total supply
    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap()
    }

    /// Get balance of an address
    pub fn balance(env: Env, address: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(address)).unwrap_or(0)
    }

    /// Transfer tokens
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        let to_balance = Self::balance(env.clone(), to.clone());

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        // Update balances
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(to_balance + amount));

        // Bump storage lifetimes
        env.storage().persistent().bump(&DataKey::Balance(from), BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        env.storage().persistent().bump(&DataKey::Balance(to), BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);

        env.events().publish((Symbol::new(&env, "transfer"),), (from, to, amount));
    }

    /// Mint new tokens (admin only)
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let current_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();
        let to_balance = Self::balance(env.clone(), to.clone());

        // Update total supply and balance
        env.storage().instance().set(&DataKey::TotalSupply, &(current_supply + amount));
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(to_balance + amount));

        env.storage().persistent().bump(&DataKey::Balance(to), BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);

        env.events().publish((Symbol::new(&env, "mint"),), (to, amount));
    }

    /// Burn tokens
    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        let current_supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap();

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        // Update total supply and balance
        env.storage().instance().set(&DataKey::TotalSupply, &(current_supply - amount));
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(from_balance - amount));

        env.storage().persistent().bump(&DataKey::Balance(from), BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);

        env.events().publish((Symbol::new(&env, "burn"),), (from, amount));
    }

    /// Approve spending allowance
    pub fn approve(env: Env, owner: Address, spender: Address, amount: i128) {
        owner.require_auth();

        env.storage().persistent().set(&DataKey::Allowance(owner.clone(), spender.clone()), &amount);
        env.storage().persistent().bump(&DataKey::Allowance(owner, spender), BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);

        env.events().publish((Symbol::new(&env, "approve"),), (owner, spender, amount));
    }

    /// Get allowance
    pub fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Allowance(owner, spender)).unwrap_or(0)
    }

    /// Transfer from approved allowance
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        let from_balance = Self::balance(env.clone(), from.clone());
        let to_balance = Self::balance(env.clone(), to.clone());

        if allowance < amount {
            panic!("Insufficient allowance");
        }
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        // Update allowance and balances
        env.storage().persistent().set(&DataKey::Allowance(from.clone(), spender), &(allowance - amount));
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(to_balance + amount));

        // Bump storage lifetimes
        env.storage().persistent().bump(&DataKey::Allowance(from.clone(), spender), BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        env.storage().persistent().bump(&DataKey::Balance(from), BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        env.storage().persistent().bump(&DataKey::Balance(to), BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);

        env.events().publish((Symbol::new(&env, "transfer_from"),), (spender, from, to, amount));
    }
}