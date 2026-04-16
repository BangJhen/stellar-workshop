#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Vec};

// Struktur data yang akan menyimpan notes
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
    // Fungsi untuk mendapatkan semua notes
    pub fn get_notes(env: Env) -> Vec<Note> {
        read_notes(&env)
    }

    // Fungsi untuk membuat note baru
    pub fn create_note(env: Env, title: String, content: String) -> String {
        // 1. ambil data notes dari storage
        let mut notes = read_notes(&env);

        // 2. Buat object note baru
        let id: u64 = env.storage().instance().get(&DataKey::NextId).unwrap_or(1);
        let note = Note { id, title, content };

        // 3. tambahkan note baru ke notes lama
        notes.push_back(note);

        // 4. simpan notes ke storage
        write_notes(&env, &notes);
        env.storage().instance().set(&DataKey::NextId, &(id + 1));

        String::from_str(&env, "Note berhasil ditambahkan")
    }

    // Fungsi untuk menghapus notes berdasarkan id
    pub fn delete_note(env: Env, id: u64) -> String {
        // 1. ambil data notes dari storage 
        let mut notes = read_notes(&env);

        // 2. cari index note yang akan dihapus menggunakan perulangan
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

mod test;













/* --- CONTOH SCRIPT ---

pub fn get_notes(env: Env) -> Vec<Note> {
    // 1. ambil data notes dari storage
    return env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
}

// Fungsi untuk membuat note baru
pub fn create_note(env: Env, title: String, content: String) -> String {
    // 1. ambil data notes dari storage
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
    
    // 2. Buat object note baru
    let note = Note {
        id: env.prng().gen::<u64>(),
        title: title,
        content: content,
    };
    
    // 3. tambahkan note baru ke notes lama
    notes.push_back(note);
    
    // 4. simpan notes ke storage
    env.storage().instance().set(&NOTE_DATA, &notes);
    
    return String::from_str(&env, "Notes berhasil ditambahkan");
}

// Fungsi untuk menghapus notes berdasarkan id
pub fn delete_note(env: Env, id: u64) -> String {
    // 1. ambil data notes dari storage 
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

    // 2. cari index note yang akan dihapus menggunakan perulangan
    for i in 0..notes.len() {
        if notes.get(i).unwrap().id == id {
            notes.remove(i);

            env.storage().instance().set(&NOTE_DATA, &notes);
            return String::from_str(&env, "Berhasil hapus notes");
        }
    }

    return String::from_str(&env, "Notes tidak ditemukan")
}


*/