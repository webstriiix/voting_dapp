use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::{state::{Candidate, Voting, CANDIDATES, CANDIDATE_SEQ, VOTES, VOTINGS, VOTING_SEQ}, ContractError};



pub fn execute_create_voting (
    deps: DepsMut,
    info: MessageInfo,
    title: String,
    description: String,
) -> Result<Response, ContractError> {
    let id = VOTING_SEQ.load(deps.storage).unwrap_or(1);
    let voting = Voting {
        id,
        creator: info.sender.clone(),
        title,
        description,
        active: true,
    };
    VOTINGS.save(deps.storage, id, &voting)?;
    VOTING_SEQ.save(deps.storage, &(id + 1))?;

    Ok(Response::new()
        .add_attribute("action", "create_voting")
        .add_attribute("id", id.to_string()))
}

pub fn execute_add_candidate (
    deps: DepsMut,
    info: MessageInfo,
    voting_id: u64,
    name: String,
    image_addr: String,
) -> Result<Response, ContractError> {
    let voting = VOTINGS.load(deps.storage, voting_id)?;

    if info.sender != voting.creator {
        return Err(ContractError::Unauthorized {  });
    }

    let cid = CANDIDATE_SEQ.load(deps.storage).unwrap_or(1);
    let candidate = Candidate {
        id: cid,
        voting_id,
        name,
        image_addr,
        vote_count: 0,
    };

    CANDIDATES.save(deps.storage, cid, &candidate)?;
    CANDIDATE_SEQ.save(deps.storage, &(cid + 1))?;

    Ok(Response::new()
        .add_attribute("action", "add_candidate")
        .add_attribute("candidate_id", cid.to_string()))
}

pub fn execute_vote(
    deps: DepsMut,
    info: MessageInfo,
    voting_id: u64,
    candidate_id: u64,
) -> Result<Response, ContractError> {
    if VOTES.may_load(deps.storage, (voting_id, info.sender.clone()))?.is_some() {
        return Err(ContractError::AlreadyVoted {  });
    }

    let mut candidate = CANDIDATES.load(deps.storage, candidate_id)?;

    if candidate.voting_id != voting_id {
        return Err(ContractError::InvalidCandidate {  });
    }

    candidate.vote_count += 1;
    CANDIDATES.save(deps.storage, candidate_id, &candidate)?;
    VOTES.save(deps.storage, (voting_id, info.sender.clone()), &candidate_id)?;

    Ok(Response::new()
        .add_attribute("action", "vote")
        .add_attribute("candidate_id", candidate_id.to_string()))
}
