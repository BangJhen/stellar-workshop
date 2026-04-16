# Stellar Workshop (Soroban Smart Contracts)

Repository ini berisi contoh dApps berbasis smart contract di jaringan Stellar (Soroban), ditulis dengan Rust.

Saat ini ada 2 aplikasi berbeda dengan task yang berbeda:

1. **Notes Contract**: Menyimpan catatan sederhana (create, read, delete).
2. **Polling Contract**: Membuat polling dan melakukan voting dengan validasi satu voter satu suara per polling.

## Tujuan Proyek

Proyek ini dibuat untuk pembelajaran dasar pengembangan smart contract Soroban, dengan fokus pada:

- Cara menyimpan state di contract storage.
- Cara membuat fungsi publik contract.
- Cara menulis unit test untuk logic contract.
- Cara menerapkan validasi sederhana (contoh: anti double vote).

## Struktur Proyek

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

## Prasyarat

- Rust toolchain (stable)
- Cargo
- (Opsional) Soroban CLI untuk deploy dan invoke di network

## Cara Menjalankan Test

Jalankan dari root repository:

```bash
cargo test -p notes
cargo test -p polling
```

## Dokumentasi Contract 1: Notes

Lokasi kode: `contracts/notes/src/lib.rs`

### Ringkasan Fungsi

1. `create_note(title, content) -> String`
  Menambahkan note baru, id dibuat incremental dari `NextId`.
2. `get_notes() -> Vec<Note>`
  Mengembalikan semua note dari storage.
3. `delete_note(id) -> String`
  Menghapus note berdasarkan id, mengembalikan pesan sukses/gagal.

### Struktur Data

```rust
pub struct Note {
   pub id: u64,
   pub title: String,
   pub content: String,
}
```

### Key Storage

- `Notes`: daftar semua note.
- `NextId`: id berikutnya untuk note baru.

### Alur Logic Sederhana

1. Baca list note dari storage.
2. Untuk create: buat objek note baru, push ke list, simpan kembali.
3. Untuk delete: loop list, cari id, remove jika ketemu, lalu simpan kembali.

## Dokumentasi Contract 2: Polling

Lokasi kode: `contracts/polling/src/lib.rs`

### Ringkasan Fungsi

1. `create_poll(question, option_a, option_b) -> u64`
  Membuat polling baru dan mengembalikan `poll_id`.
2. `get_polls() -> Vec<Poll>`
  Mengembalikan semua polling yang tersedia.
3. `vote(poll_id, option, voter) -> String`
  Menyimpan vote untuk polling tertentu.
4. `get_result(poll_id) -> PollResult`
  Menghitung total vote option A dan B untuk satu polling.

### Struktur Data

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

### Key Storage

- `Polls`: daftar polling.
- `Votes`: daftar suara.
- `NextPollId`: id polling berikutnya.

### Aturan Bisnis pada Voting

1. `voter.require_auth()`
  Vote harus disetujui oleh address pemilih.
2. Opsi vote hanya boleh `1` atau `2`.
3. `poll_id` harus valid (poll harus ada).
4. Satu voter tidak boleh vote dua kali di polling yang sama.

## Unit Test yang Tersedia

### Notes

- Create note lalu get notes.
- Delete note sukses.
- Delete note saat id tidak ditemukan.

### Polling

- Create poll lalu get polls.
- Vote valid dan hitung hasil.
- Cegah double vote.
- Tolak opsi vote tidak valid.

## Pengembangan Lanjutan (Opsional)

Ide task berikutnya:

1. **Notes**: update_note, ownership per address.
2. **Polling**: close poll, deadline voting, role admin.
3. Integrasi frontend untuk invoke contract dari wallet.

---

Jika kamu baru mulai belajar Soroban, urutan terbaik adalah:

1. Pahami kontrak Notes (state paling sederhana).
2. Lanjut ke Polling (validasi + auth + agregasi hasil).
3. Tambahkan fitur sedikit demi sedikit dan uji dengan unit test.
