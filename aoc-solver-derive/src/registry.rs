use core::{Puzzle, PuzzlePart};
use once_cell::sync::Lazy;
use std::{collections::HashSet, sync::Mutex};

pub static REGISTERED: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn register(puzzle: &Puzzle, part: &PuzzlePart) -> bool {
    let mut is_first = false;
    let key = puzzle.get_function_name(part);

    match REGISTERED.lock() {
        Ok(mut mutex) => {
            if mutex.is_empty() {
                is_first = true;
            }

            match mutex.contains(&key) {
                true => panic!("This is a duplicate!"),
                false => {
                    mutex.insert(key);
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
            panic!("REGISTERED IS DEADLOCKED!");
        }
    };

    is_first
}
