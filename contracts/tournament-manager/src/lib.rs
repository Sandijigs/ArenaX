#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec, Map};

#[contract]
pub struct TournamentManager;

#[derive(Clone)]
pub struct Tournament {
    pub id: u64,
    pub organizer: Address,
    pub name: Symbol,
    pub game_type: Symbol,
    pub entry_fee: i128,
    pub prize_pool: i128,
    pub max_participants: u32,
    pub current_participants: u32,
    pub status: TournamentStatus,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub participants: Vec<Address>,
    pub bracket: Map<Symbol, Address>, // Round -> Winner mapping
}

#[derive(Clone, Eq, PartialEq)]
pub enum TournamentStatus {
    Created,
    RegistrationOpen,
    RegistrationClosed,
    InProgress,
    Completed,
    Cancelled,
}

#[contractimpl]
impl TournamentManager {
    /// Initialize the tournament manager
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&Symbol::new(&env, "admin")) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&Symbol::new(&env, "admin"), &admin);
        env.storage().instance().set(&Symbol::new(&env, "next_tournament_id"), &1u64);
    }

    /// Create a new tournament
    pub fn create_tournament(
        env: Env,
        organizer: Address,
        name: Symbol,
        game_type: Symbol,
        entry_fee: i128,
        max_participants: u32,
        start_time: u64,
    ) -> u64 {
        organizer.require_auth();

        let tournament_id: u64 = env.storage().instance().get(&Symbol::new(&env, "next_tournament_id")).unwrap();
        env.storage().instance().set(&Symbol::new(&env, "next_tournament_id"), &(tournament_id + 1));

        let tournament = Tournament {
            id: tournament_id,
            organizer: organizer.clone(),
            name: name.clone(),
            game_type,
            entry_fee,
            prize_pool: 0,
            max_participants,
            current_participants: 0,
            status: TournamentStatus::Created,
            start_time,
            end_time: None,
            participants: Vec::new(&env),
            bracket: Map::new(&env),
        };

        env.storage().persistent().set(&Symbol::new(&env, &format!("tournament_{}", tournament_id)), &tournament);

        env.events().publish(
            (Symbol::new(&env, "tournament_created"),),
            (tournament_id, organizer, name, entry_fee)
        );

        tournament_id
    }

    /// Join a tournament
    pub fn join_tournament(env: Env, tournament_id: u64, participant: Address) {
        participant.require_auth();

        let tournament_key = Symbol::new(&env, &format!("tournament_{}", tournament_id));
        let mut tournament: Tournament = env.storage().persistent().get(&tournament_key).unwrap();

        if tournament.status != TournamentStatus::RegistrationOpen {
            panic!("Tournament not open for registration");
        }

        if tournament.current_participants >= tournament.max_participants {
            panic!("Tournament is full");
        }

        // Check if already joined
        for existing_participant in tournament.participants.iter() {
            if existing_participant == participant {
                panic!("Already joined tournament");
            }
        }

        tournament.participants.push_back(participant.clone());
        tournament.current_participants += 1;
        tournament.prize_pool += tournament.entry_fee;

        env.storage().persistent().set(&tournament_key, &tournament);

        env.events().publish(
            (Symbol::new(&env, "tournament_joined"),),
            (tournament_id, participant, tournament.entry_fee)
        );
    }

    /// Start tournament
    pub fn start_tournament(env: Env, tournament_id: u64) {
        let tournament_key = Symbol::new(&env, &format!("tournament_{}", tournament_id));
        let mut tournament: Tournament = env.storage().persistent().get(&tournament_key).unwrap();

        // Only organizer can start
        tournament.organizer.require_auth();

        if tournament.status != TournamentStatus::RegistrationClosed {
            panic!("Tournament must be closed for registration");
        }

        if tournament.current_participants < 2 {
            panic!("Tournament needs at least 2 participants");
        }

        tournament.status = TournamentStatus::InProgress;
        env.storage().persistent().set(&tournament_key, &tournament);

        env.events().publish((Symbol::new(&env, "tournament_started"),), tournament_id);
    }

    /// Report match result
    pub fn report_match_result(
        env: Env,
        tournament_id: u64,
        round: Symbol,
        winner: Address,
        loser: Address,
    ) {
        let tournament_key = Symbol::new(&env, &format!("tournament_{}", tournament_id));
        let mut tournament: Tournament = env.storage().persistent().get(&tournament_key).unwrap();

        if tournament.status != TournamentStatus::InProgress {
            panic!("Tournament not in progress");
        }

        // TODO: Validate that both players are in the tournament
        // TODO: Validate that it's actually their turn to play

        tournament.bracket.set(round.clone(), winner.clone());
        env.storage().persistent().set(&tournament_key, &tournament);

        env.events().publish(
            (Symbol::new(&env, "match_result"),),
            (tournament_id, round, winner, loser)
        );
    }

    /// Complete tournament
    pub fn complete_tournament(env: Env, tournament_id: u64, winner: Address) {
        let tournament_key = Symbol::new(&env, &format!("tournament_{}", tournament_id));
        let mut tournament: Tournament = env.storage().persistent().get(&tournament_key).unwrap();

        // Only organizer or admin can complete
        let admin: Address = env.storage().instance().get(&Symbol::new(&env, "admin")).unwrap();
        if env.invoker() != tournament.organizer && env.invoker() != admin {
            panic!("Unauthorized");
        }

        if tournament.status != TournamentStatus::InProgress {
            panic!("Tournament not in progress");
        }

        tournament.status = TournamentStatus::Completed;
        tournament.end_time = Some(env.ledger().timestamp());
        env.storage().persistent().set(&tournament_key, &tournament);

        env.events().publish(
            (Symbol::new(&env, "tournament_completed"),),
            (tournament_id, winner, tournament.prize_pool)
        );
    }

    /// Get tournament information
    pub fn get_tournament(env: Env, tournament_id: u64) -> Tournament {
        env.storage().persistent()
            .get(&Symbol::new(&env, &format!("tournament_{}", tournament_id)))
            .unwrap()
    }

    /// Get all tournaments
    pub fn get_tournaments(env: Env) -> Vec<Tournament> {
        let mut tournaments = Vec::new(&env);
        let next_id: u64 = env.storage().instance().get(&Symbol::new(&env, "next_tournament_id")).unwrap();

        for i in 1..next_id {
            if let Some(tournament) = env.storage().persistent()
                .get(&Symbol::new(&env, &format!("tournament_{}", i))) {
                tournaments.push_back(tournament);
            }
        }

        tournaments
    }
}