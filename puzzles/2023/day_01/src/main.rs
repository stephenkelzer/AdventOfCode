use core::{aoc_solver, get_count};

#[aoc_solver(2023, 16, 1)]
fn part1(input: &str) -> String {
    input.to_string()
}

#[aoc_solver(2023, 16, 2)]
fn part2(input: &str) -> String {
    input.to_string()
}

#[aoc_solver(2023, 16, 3)]
fn part2(input: &str) -> String {
    input.to_string()
}

fn main() {
    // TODO: is there a way to make this function not necessary?
    // seems like maybe going to a lib will work.

    let a = get_count!();
    assert_eq!(a, 3);

    let part_one_result = solve_2023_16_1("test_one");
    assert_eq!(part_one_result, "test_one");

    let part_two_result = solve_2023_16_2("test_two");
    assert_eq!(part_two_result, "test_two");
}
