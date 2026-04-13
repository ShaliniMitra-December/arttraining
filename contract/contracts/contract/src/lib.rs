#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Map};

#[contracttype]
#[derive(Clone)]
pub struct Art {
    pub title: String,
    pub creator: String,
    pub owner: String,
}

#[contracttype]
pub enum ArtKey {
    Art(u32),
}

#[contract]
pub struct ArtTrackingContract;

#[contractimpl]
impl ArtTrackingContract {
    // Register a new artwork
    pub fn register_art(env: Env, id: u32, title: String, creator: String, owner: String) {
        let art = Art { title, creator, owner };
        env.storage().instance().set(&ArtKey::Art(id), &art);
    }

    // Get artwork details
    pub fn get_art(env: Env, id: u32) -> Art {
        env.storage()
            .instance()
            .get(&ArtKey::Art(id))
            .unwrap()
    }

    // Transfer ownership
    pub fn transfer_ownership(env: Env, id: u32, new_owner: String) {
        let mut art: Art = env.storage()
            .instance()
            .get(&ArtKey::Art(id))
            .unwrap();

        art.owner = new_owner;
        env.storage().instance().set(&ArtKey::Art(id), &art);
    }
}