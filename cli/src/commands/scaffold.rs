use core::Puzzle;
use std::{fs::OpenOptions, io::Write, path::Path};

const MAIN_FILE_TEMPLATE: &str = r#"
aoc_solver_derive::aoc_solver!(%YEAR%, %DAY%, part1, part2);

fn part1(input: &str) -> impl std::fmt::Display {
    "TODO"
}

fn part2(input: &str) -> impl std::fmt::Display {
    "TODO"
}
"#;

const TOML_FILE_TEMPLATE: &str = r#"
[package]
name = "aoc_%YEAR%_%DAY%"
version.workspace = true
edition.workspace = true
authors.workspace = true

[[bin]]
name = "%YEAR%_%DAY%"
path = "src/main.rs"

[dependencies]
core.workspace = true
core.intertools = true
aoc-solver-derive.workspace = true
"#;

fn create_file(puzzle: &Puzzle, name: &str, content: Option<&str>) -> () {
    let path = Path::new(&puzzle.get_crate_root()).join(name);

    if let Some(prefix) = path.parent() {
        std::fs::create_dir_all(prefix).expect("Could not create directory.");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .expect("Could not create file");

    if let Some(content) = content {
        let contents = content
            .replace("%YEAR%", &format!("{:04}", &puzzle.get_year()))
            .replace("%DAY%", &format!("{:02}", puzzle.get_day()));

        file.write_all(&contents.as_bytes())
            .expect("Could not write contents to file")
    }

    println!("Created file: {:?}", path);
}

pub fn handle(puzzle: Puzzle) {
    create_file(&puzzle, "src/main.rs", Some(MAIN_FILE_TEMPLATE));
    create_file(&puzzle, "Cargo.toml", Some(TOML_FILE_TEMPLATE));
    // create_file(&puzzle, "input.txt", None);
    // create_file(&puzzle, "example.txt", None);

    println!("---");
    println!(
        "🎄 Type `cargo solve {} -y {}` to run your solution.",
        puzzle.get_day(),
        puzzle.get_year()
    );
}
