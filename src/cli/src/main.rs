mod commands;
mod templates;

use clap::{value_parser, Args, Parser, Subcommand};
use commands::{scaffold, solve};
use core::puzzle::Puzzle;
use std::ops::RangeInclusive;

const DEFAULT_YEAR: u16 = 2023; // TODO: get from environment variable
const YEAR_RANGE: RangeInclusive<i64> = 2015..=(DEFAULT_YEAR as i64);
const DAY_RANGE: RangeInclusive<i64> = 1..=25;

#[derive(Parser)]
#[command(author, about, long_about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Year Advent of Code year to run commands for
    #[arg(short, long, global = true, value_parser = value_parser!(u16).range(YEAR_RANGE))]
    year: Option<u16>,
}

#[derive(Subcommand)]
enum Commands {
    /// Scaffold the files for a new puzzle
    Scaffold(ScaffoldArgs),
    Solve(SolveArgs),
}

#[derive(Args)]
struct ScaffoldArgs {
    /// The Puzzle day
    #[arg(value_parser = value_parser!(u8).range(DAY_RANGE))]
    day: u8,
}

#[derive(Args)]
struct SolveArgs {
    /// The Puzzle day
    #[arg(value_parser = value_parser!(u8).range(DAY_RANGE))]
    day: u8,
}

fn main() {
    let cli = Cli::parse();

    let year = cli.year.unwrap_or(DEFAULT_YEAR);

    match &cli.command {
        Commands::Scaffold(ScaffoldArgs { day }) => {
            scaffold::handle(Puzzle::new(year, *day));
        }
        Commands::Solve(SolveArgs { day }) => solve::handle(Puzzle::new(year, *day), Some(1)),
    }
}
