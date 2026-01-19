#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec, Map};

#[contract]
pub struct TournamentEscrow;

#[derive(Clone)]
pub struct EscrowInfo {
    pub tournament_id: u64,
    pub buyer: Address,
    pub seller: Address,
    pub amount: i128,
    pub token: Address,
    pub status: EscrowStatus,
    pub created_at: u64,
}

#[derive(Clone, Eq, PartialEq)]
pub enum EscrowStatus {
    Pending,
    Funded,
    Completed,
    Cancelled,
    Disputed,
}

#[contractimpl]
impl TournamentEscrow {
    /// Initialize the escrow contract
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&Symbol::new(&env, "admin")) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
    }

    /// Create a new escrow for tournament entry
    pub fn create_escrow(
        env: Env,
        escrow_id: u64,
        tournament_id: u64,
        buyer: Address,
        seller: Address,
        amount: i128,
        token: Address,
    ) {
        buyer.require_auth();

        let escrow = EscrowInfo {
            tournament_id,
            buyer: buyer.clone(),
            seller,
            amount,
            token: token.clone(),
            status: EscrowStatus::Pending,
            created_at: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&Symbol::new(&env, &format!("escrow_{}", escrow_id)), &escrow);

        env.events().publish(
            (Symbol::new(&env, "escrow_created"),),
            (escrow_id, buyer, amount, token)
        );
    }

    /// Fund the escrow (transfer tokens to escrow)
    pub fn fund_escrow(env: Env, escrow_id: u64, token_contract: Address) {
        let escrow_key = Symbol::new(&env, &format!("escrow_{}", escrow_id));
        let mut escrow: EscrowInfo = env.storage().persistent().get(&escrow_key).unwrap();

        if escrow.status != EscrowStatus::Pending {
            panic!("Escrow not in pending state");
        }

        escrow.buyer.require_auth();

        // TODO: Implement token transfer from buyer to escrow contract
        // This would typically call the token contract's transfer_from function

        escrow.status = EscrowStatus::Funded;
        env.storage().persistent().set(&escrow_key, &escrow);

        env.events().publish((Symbol::new(&env, "escrow_funded"),), escrow_id);
    }

    /// Complete escrow and release funds to seller
    pub fn complete_escrow(env: Env, escrow_id: u64, token_contract: Address) {
        let escrow_key = Symbol::new(&env, &format!("escrow_{}", escrow_id));
        let mut escrow: EscrowInfo = env.storage().persistent().get(&escrow_key).unwrap();

        if escrow.status != EscrowStatus::Funded {
            panic!("Escrow not in funded state");
        }

        // Only buyer or admin can complete
        let admin: Address = env.storage().instance().get(&Symbol::new(&env, "admin")).unwrap();
        if !env.current_contract_address().eq(&escrow.buyer) && !env.current_contract_address().eq(&admin) {
            escrow.buyer.require_auth();
        }

        // TODO: Implement token transfer from escrow to seller
        // This would typically call the token contract's transfer function

        escrow.status = EscrowStatus::Completed;
        env.storage().persistent().set(&escrow_key, &escrow);

        env.events().publish((Symbol::new(&env, "escrow_completed"),), (escrow_id, escrow.seller.clone()));
    }

    /// Cancel escrow and refund buyer
    pub fn cancel_escrow(env: Env, escrow_id: u64, token_contract: Address) {
        let escrow_key = Symbol::new(&env, &format!("escrow_{}", escrow_id));
        let mut escrow: EscrowInfo = env.storage().persistent().get(&escrow_key).unwrap();

        if escrow.status != EscrowStatus::Pending && escrow.status != EscrowStatus::Funded {
            panic!("Escrow cannot be cancelled");
        }

        // Only buyer or admin can cancel
        let admin: Address = env.storage().instance().get(&Symbol::new(&env, "admin")).unwrap();
        if !env.current_contract_address().eq(&escrow.buyer) && !env.current_contract_address().eq(&admin) {
            escrow.buyer.require_auth();
        }

        if escrow.status == EscrowStatus::Funded {
            // TODO: Implement token refund to buyer
            // This would typically call the token contract's transfer function
        }

        escrow.status = EscrowStatus::Cancelled;
        env.storage().persistent().set(&escrow_key, &escrow);

        env.events().publish((Symbol::new(&env, "escrow_cancelled"),), (escrow_id, escrow.buyer.clone()));
    }

    /// Get escrow information
    pub fn get_escrow(env: Env, escrow_id: u64) -> EscrowInfo {
        env.storage().persistent()
            .get(&Symbol::new(&env, &format!("escrow_{}", escrow_id)))
            .unwrap()
    }
}