# Stellar Workshop (Soroban Smart Contracts)

This repository contains sample decentralized applications (dApps) built on the Stellar Soroban smart contract platform using Rust.

It currently includes two different applications with different tasks:

1. **Notes Contract**: Store simple notes (create, read, delete).
2. **Polling Contract**: Create polls and vote with one-voter-one-vote validation per poll.

## Project Goals

This project is designed for Soroban smart contract beginners and focuses on:

- Managing on-chain state in contract storage.
- Building public contract functions.
- Writing unit tests for contract logic.
- Applying simple business validations (for example, preventing double voting).

## Project Structure

```text
.
├── Cargo.toml
├── contracts
│   ├── notes
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       └── test.rs
│   └── polling
│       ├── Cargo.toml
│       └── src
│           ├── lib.rs
│           └── test.rs
└── README.md
```

## Prerequisites

- Rust toolchain (stable)
- Cargo
- (Optional) Soroban CLI for deployment and network invocation

## Running Tests

Run from the repository root:

```bash
cargo test -p notes
cargo test -p polling
```

## Contract 1 Documentation: Notes

Code location: `contracts/notes/src/lib.rs`

### Notes Function Summary

1. `create_note(title, content) -> String`
  Creates a new note with an incremental id from `NextId`.
2. `get_notes() -> Vec<Note>`
  Returns all notes from storage.
3. `delete_note(id) -> String`
  Deletes a note by id and returns a success or failure message.

### Notes Data Structure

```rust
pub struct Note {
   pub id: u64,
   pub title: String,
   pub content: String,
}
```

### Notes Storage Keys

- `Notes`: list of all notes.
- `NextId`: next id for new note creation.

### Notes Logic Flow

1. Read current note list from storage.
2. For create: build a note object, push to list, and save back.
3. For delete: iterate by id, remove when found, then save back.

## Contract 2 Documentation: Polling

Code location: `contracts/polling/src/lib.rs`

### Polling Function Summary

1. `create_poll(question, option_a, option_b) -> u64`
  Creates a new poll and returns `poll_id`.
2. `get_polls() -> Vec<Poll>`
  Returns all available polls.
3. `vote(poll_id, option, voter) -> String`
  Stores a vote for a specific poll.
4. `get_result(poll_id) -> PollResult`
  Calculates vote totals for option A and option B in one poll.

### Polling Data Structures

```rust
pub struct Poll {
   pub id: u64,
   pub question: String,
   pub option_a: String,
   pub option_b: String,
}

pub struct Vote {
   pub poll_id: u64,
   pub voter: Address,
   pub option: u32,
}

pub struct PollResult {
   pub poll_id: u64,
   pub total_a: u32,
   pub total_b: u32,
}
```

### Polling Storage Keys

- `Polls`: list of polls.
- `Votes`: list of all submitted votes.
- `NextPollId`: next poll id.

### Voting Business Rules

1. `voter.require_auth()`
  Vote submission must be authorized by the voter address.
2. Vote option must be either `1` or `2`.
3. `poll_id` must refer to an existing poll.
4. A voter cannot vote twice in the same poll.

## Available Unit Tests

### Notes

- Create a note and fetch notes.
- Delete note success case.
- Delete note when id is not found.

### Polling

- Create poll and fetch polls.
- Submit valid votes and calculate results.
- Prevent double vote.
- Reject invalid vote option.

## Next Development Ideas (Optional)

1. **Notes**: add `update_note`, add per-address ownership.
2. **Polling**: add poll closing, voting deadline, admin role.
3. Add frontend integration for wallet-based contract invocation.

---

If you are new to Soroban, the recommended learning order is:

1. Start with the Notes contract (simplest state management).
2. Continue with Polling (auth, validation, and result aggregation).
3. Add features incrementally and validate behavior with unit tests.
