use soroban_sdk::{Address, Symbol};

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 518400; // 30 days
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = 172800; // 10 days

#[derive(Clone)]
pub enum DataKey {
    Admin,
    Name,
    Symbol,
    Decimals,
    TotalSupply,
    Balance(Address),
    Allowance(Address, Address),
}