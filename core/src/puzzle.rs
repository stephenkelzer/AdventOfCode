use std::path::PathBuf;

use crate::config::get_latest_year;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Puzzle {
    year: u16,
    day: u8,
}

impl Puzzle {
    pub fn new(year: &u16, day: &u8) -> Self {
        let allowed_year_range = 2015..=get_latest_year();
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

    pub fn get_crate_name(&self) -> String {
        format!("aoc_{:04}_{:02}", self.year, self.day)
    }

    pub fn get_bin_name(&self) -> String {
        format!("{:04}_{:02}", self.year, self.day)
    }

    pub fn get_repository_root(&self) -> PathBuf {
        // this is the path of the dir that contains the nearest Cargo.toml file
        // in this case, it will be the core crate (because that is where this file is located).
        let core_crate_dir = env!("CARGO_MANIFEST_DIR");

        // this will be the root of the "core" crate
        PathBuf::from(core_crate_dir)
            // so let's go up one folder to bring us to the root of the repository
            .join("..")
            // then dive into the puzzle directory
            .join(format!("puzzles/{:04}/{:02}/", self.year, self.day))
    }

    pub fn get_cargo_toml_path(&self) -> PathBuf {
        PathBuf::from(&self.get_repository_root()).join("Cargo.toml")
    }

    pub fn get_main_file_path(&self) -> PathBuf {
        PathBuf::from(&self.get_repository_root()).join("src/main.rs")
    }

    pub fn get_input_file_path(&self) -> PathBuf {
        PathBuf::from(&self.get_repository_root()).join("src/input.txt")
    }

    pub fn get_example_file_path(&self) -> PathBuf {
        PathBuf::from(&self.get_repository_root()).join("src/example.txt")
    }
}
