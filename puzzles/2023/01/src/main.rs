aoc_solver_derive::aoc_solver!(2023, 1, part1, part2);

fn part1(input: &str) -> impl std::fmt::Display {
    input.to_string()
}

fn part2(input: &str) -> impl std::fmt::Display {
    input.to_string()
}

#[cfg(test)]
mod test_2023_01 {
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
