use core::Puzzle;
use std::{fs::OpenOptions, io::Write, path::PathBuf};

const MAIN_FILE_TEMPLATE: &str = r##"
aoc_solver_derive::aoc_solver!(%YEAR%, %DAY%, part1, part2);

fn part1(input: &str) -> impl std::fmt::Display {
    input.to_string()
}

fn part2(input: &str) -> impl std::fmt::Display {
    input.to_string()
}

#[cfg(test)]
mod test_%YEAR%_%DAY% {
    const EXAMPLE_INPUT: &str = r#"
abcdefg
"#;

    #[test]
    fn part_one() {
        core::runner::test(super::part1, EXAMPLE_INPUT, "abcdefg");
    }

    #[test]
    fn part_two() {
        core::runner::test(super::part2, EXAMPLE_INPUT, "abcdefg");
    }
}
"##;

const TOML_FILE_TEMPLATE: &str = r#"
[package]
name = "%CRATE_NAME%"
version.workspace = true
edition.workspace = true
authors.workspace = true

[[bin]]
name = "%BIN_NAME%"
path = "src/main.rs"

[dependencies]
core.workspace = true
core.intertools = true
aoc-solver-derive.workspace = true
"#;

fn create_file(puzzle: &Puzzle, file_path: &PathBuf, content: Option<&str>) -> () {
    if let Some(prefix) = file_path.parent() {
        std::fs::create_dir_all(prefix).expect("Could not create directory.");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&file_path)
        .expect("Could not create file");

    if let Some(content) = content {
        let contents = content
            .replace("%YEAR%", &format!("{:04}", &puzzle.get_year()))
            .replace("%DAY%", &format!("{:02}", puzzle.get_day()))
            .replace("%CRATE_NAME%", &puzzle.get_crate_name())
            .replace("%BIN_NAME%", &puzzle.get_bin_name());

        file.write_all(&contents.as_bytes())
            .expect("Could not write contents to file")
    }

    println!("Created file: {:?}", file_path);
}

pub fn handle(puzzle: Puzzle) {
    create_file(
        &puzzle,
        &puzzle.get_main_file_path(),
        Some(MAIN_FILE_TEMPLATE),
    );
    create_file(
        &puzzle,
        &puzzle.get_cargo_toml_path(),
        Some(TOML_FILE_TEMPLATE),
    );
    create_file(&puzzle, &puzzle.get_input_file_path(), None);
    // create_file(&puzzle, "example.txt", None);

    println!("---");
    println!(
        "🎄 Type `cargo solve {} -y {}` to run your solution.",
        puzzle.get_day(),
        puzzle.get_year()
    );
}
