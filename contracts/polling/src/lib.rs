#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Poll {
    pub id: u64,
    pub question: String,
    pub option_a: String,
    pub option_b: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vote {
    pub poll_id: u64,
    pub voter: Address,
    pub option: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PollResult {
    pub poll_id: u64,
    pub total_a: u32,
    pub total_b: u32,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Polls,
    Votes,
    NextPollId,
}

fn read_polls(env: &Env) -> Vec<Poll> {
    env.storage()
        .instance()
        .get(&DataKey::Polls)
        .unwrap_or(Vec::new(env))
}

fn write_polls(env: &Env, polls: &Vec<Poll>) {
    env.storage().instance().set(&DataKey::Polls, polls);
}

fn read_votes(env: &Env) -> Vec<Vote> {
    env.storage()
        .instance()
        .get(&DataKey::Votes)
        .unwrap_or(Vec::new(env))
}

fn write_votes(env: &Env, votes: &Vec<Vote>) {
    env.storage().instance().set(&DataKey::Votes, votes);
}

fn poll_exists(polls: &Vec<Poll>, poll_id: u64) -> bool {
    for i in 0..polls.len() {
        if let Some(poll) = polls.get(i) {
            if poll.id == poll_id {
                return true;
            }
        }
    }

    false
}

#[contract]
pub struct PollingContract;

#[contractimpl]
impl PollingContract {
    pub fn create_poll(env: Env, question: String, option_a: String, option_b: String) -> u64 {
        let mut polls = read_polls(&env);
        let poll_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::NextPollId)
            .unwrap_or(1);

        let poll = Poll {
            id: poll_id,
            question,
            option_a,
            option_b,
        };

        polls.push_back(poll);
        write_polls(&env, &polls);
        env.storage()
            .instance()
            .set(&DataKey::NextPollId, &(poll_id + 1));

        poll_id
    }

    pub fn get_polls(env: Env) -> Vec<Poll> {
        read_polls(&env)
    }

    pub fn vote(env: Env, poll_id: u64, option: u32, voter: Address) -> String {
        voter.require_auth();

        if option != 1 && option != 2 {
            return String::from_str(&env, "Opsi vote tidak valid");
        }

        let polls = read_polls(&env);
        if !poll_exists(&polls, poll_id) {
            return String::from_str(&env, "Poll tidak ditemukan");
        }

        let mut votes = read_votes(&env);
        for i in 0..votes.len() {
            if let Some(existing_vote) = votes.get(i) {
                if existing_vote.poll_id == poll_id && existing_vote.voter == voter {
                    return String::from_str(&env, "Voter sudah memilih");
                }
            }
        }

        votes.push_back(Vote {
            poll_id,
            voter,
            option,
        });
        write_votes(&env, &votes);

        String::from_str(&env, "Vote berhasil disimpan")
    }

    pub fn get_result(env: Env, poll_id: u64) -> PollResult {
        let votes = read_votes(&env);
        let mut total_a: u32 = 0;
        let mut total_b: u32 = 0;

        for i in 0..votes.len() {
            if let Some(vote) = votes.get(i) {
                if vote.poll_id == poll_id {
                    if vote.option == 1 {
                        total_a += 1;
                    } else if vote.option == 2 {
                        total_b += 1;
                    }
                }
            }
        }

        PollResult {
            poll_id,
            total_a,
            total_b,
        }
    }
}

#[cfg(test)]
mod test;
