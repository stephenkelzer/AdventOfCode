use once_cell::sync::Lazy;
use std::{collections::HashSet, sync::Mutex};

pub static REGISTERED: Lazy<Mutex<HashSet<(u16, u8, u8)>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));

pub fn register_if_unique(key: (u16, u8, u8)) {
    match REGISTERED.lock() {
        Ok(mut mutex) => match mutex.contains(&key) {
            true => panic!(
                "Value '{:?}' has already been used in a previous invocation of the macro!",
                key
            ),
            false => {
                mutex.insert(key);
            }
        },
        Err(e) => {
            println!("{:?}", e);
            panic!("REGISTERED IS DEADLOCKED!");
        }
    };
}

pub fn get_registery_count() -> usize {
    match REGISTERED.try_lock() {
        Ok(mutex) => mutex.len(),
        Err(e) => {
            println!("{:?}", e);
            panic!("REGISTERED IS DEADLOCKED! (this likely means that there is a duplicate aoc_solver macro)");
        }
    }
}
