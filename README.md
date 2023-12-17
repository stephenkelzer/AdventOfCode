# Advent of Code 2023

![Tests](https://github.com/stephenkelzer/aoc2023/actions/workflows/lint_and_test.yml/badge.svg)

This is my codebase for the 2023 [Advent of Code](https://adventofcode.com/). This is a simple Rust workflows project separating each day of the event into it's own workspace. There are no binaries in this project, so running `cargo run` will not work. Instead, each day's workspace is a library and has unit tests to check their answer. Once I have successfully completed the puzzle, I update the corresponding unit test's assertion to be correct and then commit the code.

#### Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install)

## How to

Run all tests:

```bash
cargo test --release
```

Run a specific days tests:

```bash
cargo test -p day_09 --release
```

# TODO:

support:

```
cargo solve all         // solves EVERYTHING
cargo solve 5           // both parts of day 5 (assumes current year)
cargo solve 5.1         // part 1 of day 5 (assumes current year)
cargo solve 2023.5      // fifth day of 2023 (both parts)
cargo solve 2023.5.2    // part 2 of the fifth day of 2023
cargo solve 2022        // solves all puzzles in 2022 year

cargo scaffold 7        // scaffolds the files needed for day 7 (assumes current year)
cargo scaffold 2023.7   // scaffolds the files needed for day 7 of 2023

```

stretch goals:

- add a flag to the scaffold command that downloads data from adventofcode.com
- submit puzzles to the aoc api
