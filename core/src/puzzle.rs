use std::path::{Path, PathBuf};

use crate::Configuration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Puzzle {
    year: u16,
    day: u8,
}

impl Puzzle {
    pub fn new(year: &u16, day: &u8) -> Self {
        let allowed_year_range = 2015..=Configuration::new().latest_year;
        let allowed_day_range = 1..=25;

        if !allowed_year_range.contains(year) {
            panic!("Year out of bounds!")
        } else if !allowed_day_range.contains(day) {
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
