use crate::puzzle_part::PuzzlePart;

pub struct Puzzle {
    year: u16,
    day: u8,
}

impl Puzzle {
    pub fn new(year: u16, day: u8) -> Self {
        if year < 2015 || year > 2023 || day > 25 {
            // TODO: validate actual current year instead of hardcoding 2023
            panic!("TODO")
        }

        Self { year, day }
    }

    pub fn get_crate_root(&self) -> String {
        format!("puzzles/{:04}/{:02}/", self.year, self.day)
    }

    pub fn get_crate_name(&self) -> String {
        format!("aoc_{:04}_{:02}", self.year, self.day)
    }

    pub fn get_binary_name(&self) -> String {
        format!("aoc_solver_{:04}_{:02}", self.year, self.day)
    }

    pub fn get_function_name(&self, part: PuzzlePart) -> String {
        format!(
            "solver_{:04}_{:02}_{}",
            self.year,
            self.day,
            match part {
                PuzzlePart::One => 1,
                PuzzlePart::Two => 2,
            }
        )
    }
}
