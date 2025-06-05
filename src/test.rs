#[cfg(test)]
mod test {
    use cosmwasm_std::{
        from_json,
        testing::{mock_dependencies, mock_env, mock_info},
        DepsMut,
    };

    use crate::{
        contract::{execute, instantiate, query},
        msg::{ExecuteMsg, GetVotingResponse, InstantiateMsg, QueryMsg},
        state::{CANDIDATES, VOTINGS},
    };

    fn instantiate_contract(deps: DepsMut) {
        let info = mock_info("creator", &[]);
        let msg = InstantiateMsg {};
        let env = mock_env();
        instantiate(deps, env, info, msg).unwrap();
    }

    #[test]
    fn test_create_voting() {
        let mut deps = mock_dependencies();
        instantiate_contract(deps.as_mut());

        let msg = ExecuteMsg::CreateVoting {
            title: "Test Vote".into(),
            description: "Test Vote Description".into(),
        };
        let info = mock_info("Alice", &[]);
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        assert_eq!(res.attributes[0], ("action", "create_voting"));
        let voting = VOTINGS.load(&deps.storage, 1).unwrap();
        assert_eq!(voting.title, "Test Vote");
        assert_eq!(voting.creator, info.sender);
    }

    #[test]
    fn test_query_list_votings() {
        let mut deps = mock_dependencies();
        instantiate_contract(deps.as_mut());

        let creator = mock_info("alice", &[]);

        // Create multiple votings
        for i in 1..=3 {
            let _ = execute(
                deps.as_mut(),
                mock_env(),
                creator.clone(),
                ExecuteMsg::CreateVoting {
                    title: format!("Voting {}", i),
                    description: format!("Description {}", i),
                },
            )
            .unwrap();
        }

        // Query for list of all votings
        let bin = query(deps.as_ref(), mock_env(), QueryMsg::GetListVoting {}).unwrap();

        let result: crate::msg::GetListVotingResponse = from_json(bin).unwrap();
        assert_eq!(result.votings.len(), 3);
        assert_eq!(result.votings[0].title, "Voting 1");
        assert_eq!(result.votings[1].title, "Voting 2");
        assert_eq!(result.votings[2].title, "Voting 3");
    }

    #[test]
    fn test_add_candidates() {
        let mut deps = mock_dependencies();
        instantiate_contract(deps.as_mut());

        let creator = mock_info("alice", &[]);

        let create_voting_msg = ExecuteMsg::CreateVoting {
            title: "Test Voting".to_string(),
            description: "Just a test".to_string(),
        };

        let _ = execute(
            deps.as_mut(),
            mock_env(),
            creator.clone(),
            create_voting_msg,
        )
        .expect("create voting should work");

        let add_candidate_msg = ExecuteMsg::AddCandidate {
            voting_id: 1,
            name: "Candidate A".to_string(),
            image_addr: "https://gratisography.com/wp-content/uploads/2025/01/gratisography-dog-vacation-800x525.jpg".to_string(),
        };

        let res = execute(
            deps.as_mut(),
            mock_env(),
            creator.clone(),
            add_candidate_msg,
        )
        .expect("add candidate should work");

        assert_eq!(res.attributes[0], ("action", "add_candidate"));

        let candidate = CANDIDATES.load(&deps.storage, 1).unwrap();
        assert_eq!(candidate.name, "Candidate A");
    }

    #[test]
    fn test_vote() {
        let mut deps = mock_dependencies();
        instantiate_contract(deps.as_mut());

        let creator = mock_info("alice", &[]);
        let voter = mock_info("bob", &[]);

        execute(
            deps.as_mut(),
            mock_env(),
            creator.clone(),
            ExecuteMsg::CreateVoting {
                title: "Voting".into(),
                description: "desc".into(),
            },
        )
        .unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            creator.clone(),
            ExecuteMsg::AddCandidate {
                voting_id: 1,
                name: "Candidate A".into(),
                image_addr: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTtnvAOajH9gS4C30cRF7rD_voaTAKly2Ntaw&s".into()
            }
        ).unwrap();

        let msg = ExecuteMsg::Vote {
            voting_id: 1,
            candidate_id: 1,
        };
        let res = execute(deps.as_mut(), mock_env(), voter.clone(), msg).unwrap();
        assert_eq!(res.attributes[0], ("action", "vote"));

        let candidate = CANDIDATES.load(&deps.storage, 1).unwrap();
        assert_eq!(candidate.vote_count, 1);
    }

    #[test]
    fn test_query_voting() {
        let mut deps = mock_dependencies();
        instantiate_contract(deps.as_mut());

        let creator = mock_info("alice", &[]);
        execute(
            deps.as_mut(),
            mock_env(),
            creator.clone(),
            ExecuteMsg::CreateVoting {
                title: "Voting Q".into(),
                description: "desc".into(),
            },
        )
        .unwrap();

        let bin = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetVoting { voting_id: 1 },
        )
        .unwrap();

        let result: GetVotingResponse = from_json(bin).unwrap();
        assert_eq!(result.voting.title, "Voting Q");
    }
}
