mod commands;

use clap::{Args, Parser, Subcommand};
use core::{Configuration, Puzzle};

use crate::commands::scaffold;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Year Advent of Code year to run commands for
    #[arg(short, long, global = true)]
    year: Option<u16>,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold the files for a new puzzle
    Scaffold(ScaffoldArgs),
}

#[derive(Args)]
struct ScaffoldArgs {
    /// The Puzzle day
    #[arg()]
    day: u8,
}

fn main() {
    let Cli { command, year, .. } = Cli::parse();

    let year = year.unwrap_or(Configuration::new().default_year);

    match &command {
        Commands::Scaffold(ScaffoldArgs { day }) => {
            let puzzle = Puzzle::new(&year, &day);
            scaffold::handle(puzzle);
        }
    }
}
