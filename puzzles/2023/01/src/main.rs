core::solver!(2023, 1, part1, part2, 26, 23);

fn part1(input: &str) -> impl std::fmt::Display {
    input.len()
}

fn part2(input: &str) -> impl std::fmt::Display {
    input.replace("abc", "").len()
}
