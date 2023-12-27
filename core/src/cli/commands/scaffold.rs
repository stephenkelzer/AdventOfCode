use core::{config::get_session_token, Puzzle};
use std::{fs::OpenOptions, io::Write, path::PathBuf};

const MAIN_FILE_TEMPLATE: &str = r##"
core::solution!(%YEAR%, %DAY%, part1, part2);

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
itertools.workspace = true
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

fn try_download_input(puzzle: &Puzzle) {
    let request = reqwest::blocking::Client::new()
        .get(format!(
            "https://adventofcode.com/{:?}/day/{:?}/input",
            puzzle.get_year(),
            puzzle.get_day()
        ))
        .header(
            reqwest::header::COOKIE,
            format!("session={};", get_session_token()),
        );

    println!("Downloading Input File Contents...");

    match request.send() {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => match response.text() {
                Ok(content) => {
                    let trimmed_content = content.trim_start_matches("\n").trim_end_matches("\n");

                    OpenOptions::new()
                        .write(true)
                        .create_new(false)
                        .open(&puzzle.get_input_file_path())
                        .expect("Could open input file for writing.")
                        .write_all(&trimmed_content.as_bytes())
                        .expect("Failed to write contents to input file.");

                    println!("Input File Updated!");
                }
                Err(_) => println!("input download request failed! Could not download contents."),
            },
            _ => {
                println!("input download request failed! Check your `.env` file for a valid session token.")
            }
        },
        Err(_) => println!("input download request failed!"),
    }
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

    try_download_input(&puzzle);

    println!("---");
    println!(
        "🎄 Type `cargo solve {} -y {}` to run your solution.",
        puzzle.get_day(),
        puzzle.get_year()
    );
}
