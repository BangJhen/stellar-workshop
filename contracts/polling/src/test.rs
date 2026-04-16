extern crate std;

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn create_poll_and_get_polls() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PollingContract);
    let client = PollingContractClient::new(&env, &contract_id);

    let poll_id = client.create_poll(
        &String::from_str(&env, "Makan favorit?"),
        &String::from_str(&env, "Nasi Goreng"),
        &String::from_str(&env, "Mie Ayam"),
    );

    assert_eq!(poll_id, 1);

    let polls = client.get_polls();
    assert_eq!(polls.len(), 1);

    let poll = polls.get(0).unwrap();
    assert_eq!(poll.id, 1);
    assert_eq!(poll.question, String::from_str(&env, "Makan favorit?"));
    assert_eq!(poll.option_a, String::from_str(&env, "Nasi Goreng"));
    assert_eq!(poll.option_b, String::from_str(&env, "Mie Ayam"));
}

#[test]
fn vote_once_and_get_result() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, PollingContract);
    let client = PollingContractClient::new(&env, &contract_id);

    let poll_id = client.create_poll(
        &String::from_str(&env, "Backend favorit?"),
        &String::from_str(&env, "Rust"),
        &String::from_str(&env, "Go"),
    );

    let voter_1 = Address::generate(&env);
    let voter_2 = Address::generate(&env);

    let vote_msg_1 = client.vote(&poll_id, &1, &voter_1);
    let vote_msg_2 = client.vote(&poll_id, &2, &voter_2);

    assert_eq!(vote_msg_1, String::from_str(&env, "Vote berhasil disimpan"));
    assert_eq!(vote_msg_2, String::from_str(&env, "Vote berhasil disimpan"));

    let result = client.get_result(&poll_id);
    assert_eq!(result.total_a, 1);
    assert_eq!(result.total_b, 1);
}

#[test]
fn prevent_double_vote() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, PollingContract);
    let client = PollingContractClient::new(&env, &contract_id);

    let poll_id = client.create_poll(
        &String::from_str(&env, "Pilih editor?"),
        &String::from_str(&env, "VS Code"),
        &String::from_str(&env, "Neovim"),
    );

    let voter = Address::generate(&env);

    let first_vote = client.vote(&poll_id, &1, &voter);
    let second_vote = client.vote(&poll_id, &2, &voter);

    assert_eq!(first_vote, String::from_str(&env, "Vote berhasil disimpan"));
    assert_eq!(second_vote, String::from_str(&env, "Voter sudah memilih"));

    let result = client.get_result(&poll_id);
    assert_eq!(result.total_a, 1);
    assert_eq!(result.total_b, 0);
}

#[test]
fn reject_invalid_option() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, PollingContract);
    let client = PollingContractClient::new(&env, &contract_id);

    let poll_id = client.create_poll(
        &String::from_str(&env, "Framework web?"),
        &String::from_str(&env, "Actix"),
        &String::from_str(&env, "Axum"),
    );

    let voter = Address::generate(&env);
    let msg = client.vote(&poll_id, &3, &voter);

    assert_eq!(msg, String::from_str(&env, "Opsi vote tidak valid"));
}
