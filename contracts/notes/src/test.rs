extern crate std;

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn create_and_get_note() {
	let env = Env::default();
	let contract_id = env.register_contract(None, NotesContract);
	let client = NotesContractClient::new(&env, &contract_id);

	let title = String::from_str(&env, "Belajar Soroban");
	let content = String::from_str(&env, "Kontrak pertama saya");

	let result = client.create_note(&title, &content);
	assert_eq!(result, String::from_str(&env, "Note berhasil ditambahkan"));

	let notes = client.get_notes();
	assert_eq!(notes.len(), 1);

	let note = notes.get(0).unwrap();
	assert_eq!(note.id, 1);
	assert_eq!(note.title, title);
	assert_eq!(note.content, content);
}

#[test]
fn delete_note_success() {
	let env = Env::default();
	let contract_id = env.register_contract(None, NotesContract);
	let client = NotesContractClient::new(&env, &contract_id);

	client.create_note(
		&String::from_str(&env, "Judul 1"),
		&String::from_str(&env, "Isi 1"),
	);
	client.create_note(
		&String::from_str(&env, "Judul 2"),
		&String::from_str(&env, "Isi 2"),
	);

	let delete_msg = client.delete_note(&1);
	assert_eq!(delete_msg, String::from_str(&env, "Berhasil hapus note"));

	let notes = client.get_notes();
	assert_eq!(notes.len(), 1);
	assert_eq!(notes.get(0).unwrap().id, 2);
}

#[test]
fn delete_note_not_found() {
	let env = Env::default();
	let contract_id = env.register_contract(None, NotesContract);
	let client = NotesContractClient::new(&env, &contract_id);

	let delete_msg = client.delete_note(&999);
	assert_eq!(delete_msg, String::from_str(&env, "Note tidak ditemukan"));
}
