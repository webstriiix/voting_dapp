use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Voting {
    pub id: u64,
    pub creator: Addr,
    pub title: String,
    pub description: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Candidate {
    pub id: u64,
    pub voting_id: u64,
    pub name: String,
    pub image_addr: String,
    pub vote_count: u64,
}

pub const VOTING_SEQ: Item<u64> = Item::new("voting_seq");
pub const CANDIDATE_SEQ: Item<u64> = Item::new("candidate_seq");

pub const VOTINGS: Map<u64, Voting> = Map::new("votings");
pub const CANDIDATES: Map<u64, Candidate> = Map::new("candidates");
pub const VOTES: Map<(u64, Addr), u64> = Map::new("votes");
