#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Vec, Address};

// ===== STRUCT =====
#[contracttype]
#[derive(Clone, Debug)]
pub struct Menu {
    pub id: u64,
    pub name: String,
    pub price: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Order {
    pub user: Address,
    pub menu_id: u64,
    pub quantity: u64,
}

// ===== STORAGE KEY =====
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    MenuList,
    OrderList,
}

// ===== CONTRACT =====
#[contract]
pub struct CafeContract;

#[contractimpl]
impl CafeContract {

    // Tambah menu
    pub fn add_menu(env: Env, id: u64, name: String, price: u64) {
        let mut menus: Vec<Menu> = env
            .storage()
            .instance()
            .get(&DataKey::MenuList)
            .unwrap_or(Vec::new(&env));

        let menu = Menu { id, name, price };
        menus.push_back(menu);

        env.storage().instance().set(&DataKey::MenuList, &menus);
    }

    // Ambil menu
    pub fn get_menus(env: Env) -> Vec<Menu> {
        env.storage()
            .instance()
            .get(&DataKey::MenuList)
            .unwrap_or(Vec::new(&env))
    }

    // Order
    pub fn order_menu(env: Env, user: Address, menu_id: u64, quantity: u64) {
        user.require_auth();

        let mut orders: Vec<Order> = env
            .storage()
            .instance()
            .get(&DataKey::OrderList)
            .unwrap_or(Vec::new(&env));

        let order = Order {
            user,
            menu_id,
            quantity,
        };

        orders.push_back(order);

        env.storage().instance().set(&DataKey::OrderList, &orders);
    }

    // Ambil order
    pub fn get_orders(env: Env) -> Vec<Order> {
        env.storage()
            .instance()
            .get(&DataKey::OrderList)
            .unwrap_or(Vec::new(&env))
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