use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Puzzle {
    year: u16,
    day: u8,
}

impl Puzzle {
    pub fn new(year: &u16, day: &u8) -> Self {
        if !(&2015..=&2023).contains(&year) {
            // TODO: validate actual current year instead of hardcoding 2023
            panic!("Year out of bounds!")
        } else if !(&1..=&25).contains(&day) {
            panic!("Day out of bounds!")
        }

        Self {
            year: *year,
            day: *day,
        }
    }

    pub fn get_year(&self) -> u16 {
        self.year
    }

    pub fn get_day(&self) -> u8 {
        self.day
    }

    pub fn get_crate_root(&self) -> String {
        format!("puzzles/{:04}/{:02}/", self.year, self.day)
    }

    pub fn get_crate_name(&self) -> String {
        format!("aoc_{:04}_{:02}", self.year, self.day)
    }

    pub fn get_bin_name(&self) -> String {
        format!("{:04}_{:02}", self.year, self.day)
    }

    pub fn get_cargo_toml_path(&self) -> PathBuf {
        Path::new(&self.get_crate_root()).join("Cargo.toml")
    }

    pub fn get_main_file_path(&self) -> PathBuf {
        Path::new(&self.get_crate_root()).join("src/main.rs")
    }

    pub fn get_input_file_path(&self) -> PathBuf {
        Path::new(&self.get_crate_root()).join("src/input.txt")
    }
}
