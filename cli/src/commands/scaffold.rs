use std::{
    fs::OpenOptions,
    io::{Error, Write},
    path::Path,
};

use core::Puzzle;

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

fn safe_create_file(puzzle: &Puzzle, name: &str, content: Option<&str>) -> () {
    let path = Path::new(&puzzle.get_crate_root()).join(name);
    // let path = format!("src/puzzles/{}/{}/{}.rs", puzzle.year, puzzle.day, name);

    let try_run = || -> Result<(), Error> {
        let file_path = Path::new(&path);

        if let Some(prefix) = file_path.parent() {
            std::fs::create_dir_all(prefix)?;
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_path)?;

        if let Some(content) = content {
            let contents = content
                .replace("%YEAR%", &format!("{:04}", &puzzle.get_year()))
                .replace("%DAY%", &format!("{:02}", puzzle.get_day()));

            file.write_all(&contents.as_bytes())?
        }

        Ok(())
    };

    if let Err(e) = try_run() {
        eprintln!("Failed to create file: {path:?}");
        panic!("{e}");
    } else {
        println!("Wrote file: {path:?}");
    }
}

pub fn handle(puzzle: Puzzle) {
    let create_file_placeholder =
        |name: &str, content: Option<&str>| -> () { safe_create_file(&puzzle, name, content) };

    create_file_placeholder("src/main.rs", Some(MAIN_FILE_TEMPLATE));
    create_file_placeholder("Cargo.toml", Some(TOML_FILE_TEMPLATE));
    // create_file_placeholder("input.txt", None);
    // create_file_placeholder("example.txt", None);

    println!("---");
    println!(
        "🎄 Type `cargo solve {} -y {}` to run your solution.",
        puzzle.get_day(),
        puzzle.get_year()
    );
}
