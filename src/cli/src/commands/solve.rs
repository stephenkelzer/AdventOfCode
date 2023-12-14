use core::puzzle::Puzzle;
use std::process::{Command, Stdio};

pub fn handle(puzzle: Puzzle, part: Option<u8>) {
    if !puzzle.is_scaffolded() {
        panic!("NOT SCAFFOLDED!")
    }

    let mut cmd_args = vec![
        "run".to_string(),
        "--bin".to_string(),
        puzzle.get_crate_name(),
        "--release".to_string(),
    ];

    cmd_args.push("--".to_string());

    if let Some(part) = part {
        cmd_args.push("--submit".to_string());
        cmd_args.push(part.to_string());
    }

    // if time {
    //     cmd_args.push("--time".to_string());
    // }

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
