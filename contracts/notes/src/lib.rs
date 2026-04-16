#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Vec};

// Struktur data yang akan menyimpan notes.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
}

// Key storage instance untuk daftar notes dan id berikutnya.
#[contracttype]
#[derive(Clone)]
enum DataKey {
    Notes,
    NextId,
}

fn read_notes(env: &Env) -> Vec<Note> {
    env.storage()
        .instance()
        .get(&DataKey::Notes)
        .unwrap_or(Vec::new(env))
}

fn write_notes(env: &Env, notes: &Vec<Note>) {
    env.storage().instance().set(&DataKey::Notes, notes);
}

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {
    // Mengambil seluruh daftar notes.
    pub fn get_notes(env: Env) -> Vec<Note> {
        read_notes(&env)
    }

    // Menambah satu note baru ke storage.
    pub fn create_note(env: Env, title: String, content: String) -> String {
        let mut notes = read_notes(&env);

        // ID dibuat incremental agar mudah dipahami.
        let id: u64 = env.storage().instance().get(&DataKey::NextId).unwrap_or(1);
        let note = Note { id, title, content };

        notes.push_back(note);
        write_notes(&env, &notes);
        env.storage().instance().set(&DataKey::NextId, &(id + 1));

        String::from_str(&env, "Note berhasil ditambahkan")
    }

    // Menghapus note berdasarkan id.
    pub fn delete_note(env: Env, id: u64) -> String {
        let mut notes = read_notes(&env);

        for i in 0..notes.len() {
            if let Some(note) = notes.get(i) {
                if note.id == id {
                    notes.remove(i);
                    write_notes(&env, &notes);
                    return String::from_str(&env, "Berhasil hapus note");
                }
            }
        }

        String::from_str(&env, "Note tidak ditemukan")
    }
}

#[cfg(test)]
mod test;
