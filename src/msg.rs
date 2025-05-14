use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::{Candidate, Voting};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateVoting { title: String, description: String },
    AddCandidate { voting_id: u64, name: String, image_addr: String },
    Vote { voting_id: u64, candidate_id: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetVotingResponse)]
    GetVoting { voting_id: u64 },
    #[returns(GetListCandidateResponse)]
    ListCandidates { voting_id: u64 },
    #[returns(GetUserVoteResponse)]
    GetUserVote { voting_id: u64, user: String },
}

#[cw_serde]
pub struct GetVotingResponse {
    pub voting: Voting,
}

#[cw_serde]
pub struct GetListVotingResponse {
    pub votings: Vec<Voting>,
}

#[cw_serde]
pub struct GetListCandidateResponse {
    pub candidates: Vec<Candidate>,
}

#[cw_serde]
pub struct GetListResultResponse {
    pub results: Vec<Candidate>,
}

#[cw_serde]
pub struct GetUserVoteResponse {
    pub candidate_id: Option<u64>,
}
