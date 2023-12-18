use aoc_solver_derive::{aoc_solver, get_count};
use std::env;

#[aoc_solver(2023, 2, 1)]
fn part1(input: &str) -> String {
    input.to_string()
}

#[aoc_solver(2023, 2, 2)]
fn part2(input: &str) -> String {
    input.to_string()
}

fn main() {
    // TODO: is there a way to make this function not necessary?
    // seems like maybe going to a lib will work.
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    // can call this one with:
    //  cargo run --bin 2023_02
    //  cargo run --bin 2023_02 1 (to run a single part?)

    let a = get_count!();
    assert_eq!(a, 2);

    let part_one_result = solve_2023_2_1("test_one");
    assert_eq!(part_one_result, "test_one");

    let part_two_result = solve_2023_2_2("test_two");
    assert_eq!(part_two_result, "test_two");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true)
    }
}

#[cfg(test)]
mod tests2 {
    #[test]
    fn test() {
        assert!(true)
    }
}
