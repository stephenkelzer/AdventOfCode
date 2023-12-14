use std::{
    fs::OpenOptions,
    io::{Error, Write},
    path::Path,
};

use core::puzzle::Puzzle;

use crate::templates::{CARGO_TOML_TEMPLATE, MODULE_TEMPLATE};

fn create_file(puzzle: &Puzzle, name: &str, content: Option<&str>) -> () {
    let path = format!(
        "src/challenges/{}/{}/{}",
        puzzle.get_year_display(),
        puzzle.get_day_display(),
        name
    );

    let try_create = || -> Result<(), Error> {
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
                .replace("%YEAR_NUMBER%", &puzzle.get_year_display())
                .replace("%DAY_NUMBER%", &puzzle.get_day_display())
                .replace("%CRATE_NAME%", &puzzle.get_crate_name());

            file.write_all(&contents.as_bytes())?
        }

        Ok(())
    };

    if let Err(e) = try_create() {
        eprintln!("Failed to create file: {path}");
        panic!("{e}");
    } else {
        println!("Wrote file: {path}");
    }
}

pub fn handle(puzzle: Puzzle) {
    create_file(&puzzle, "src/mod.rs", Some(MODULE_TEMPLATE));
    create_file(&puzzle, "src/input.txt", None);
    create_file(&puzzle, "src/example.txt", None);
    create_file(&puzzle, "Cargo.toml", Some(CARGO_TOML_TEMPLATE));

    println!("---");
    println!(
        "🎄 Type `cargo solve {}` to run your solution.",
        puzzle.get_day_value()
    );
}
