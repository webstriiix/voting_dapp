use cosmwasm_std::{Addr, Deps, Order, StdResult};

use crate::{
    msg::{
        GetListCandidateResponse, GetListVotingResponse, GetUserVoteResponse, GetVotingResponse,
    },
    state::{Voting, CANDIDATES, VOTES, VOTINGS},
};

pub fn list_votings(deps: Deps) -> StdResult<GetListVotingResponse> {
    let votings: Vec<Voting> = VOTINGS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| item.map(|(_, v)| v))
        .collect::<StdResult<_>>()?;

    Ok(GetListVotingResponse { votings })
}

pub fn query_voting(deps: Deps, voting_id: u64) -> StdResult<GetVotingResponse> {
    let voting = VOTINGS.load(deps.storage, voting_id)?;

    Ok(GetVotingResponse { voting })
}

pub fn query_candidates(deps: Deps, voting_id: u64) -> StdResult<GetListCandidateResponse> {
    let candidates: Vec<_> = CANDIDATES
        .range(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| {
            let (_, candidate) = item.ok()?;
            if candidate.voting_id == voting_id {
                Some(candidate)
            } else {
                None
            }
        })
        .collect();

    Ok(GetListCandidateResponse { candidates })
}

pub fn query_vote(deps: Deps, voting_id: u64, user: Addr) -> StdResult<GetUserVoteResponse> {
    let vote = VOTES.may_load(deps.storage, (voting_id, user))?;
    Ok(GetUserVoteResponse { candidate_id: vote })
}
