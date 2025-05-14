#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{execute_add_candidate, execute_create_voting, execute_vote};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_candidates, query_vote, query_voting};
use crate::state::{CANDIDATE_SEQ, VOTING_SEQ};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:voting_dapp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    VOTING_SEQ.save(deps.storage, &1)?;
    CANDIDATE_SEQ.save(deps.storage, &1)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateVoting { title, description } => {
            execute_create_voting(deps, info, title, description)
        }
        ExecuteMsg::AddCandidate { voting_id, name, image_addr } => {
            execute_add_candidate(deps, info, voting_id, name, image_addr)
        }
        ExecuteMsg::Vote {
            voting_id,
            candidate_id,
        } => execute_vote(deps, info, voting_id, candidate_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetVoting { voting_id } => to_json_binary(&query_voting(deps, voting_id)?),
        QueryMsg::ListCandidates { voting_id } => {
            to_json_binary(&query_candidates(deps, voting_id)?)
        }
        QueryMsg::GetUserVote { voting_id, user } => {
            let addr = deps.api.addr_validate(&user)?;
            to_json_binary(&query_vote(deps, voting_id, addr)?)
        }
    }
}

