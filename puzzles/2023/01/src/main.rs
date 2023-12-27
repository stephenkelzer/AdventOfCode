core::solver!(2023, 1, part_one, part_two, 26, 23);

fn part_one(input: &str) -> impl std::fmt::Display {
    input.len()
}

fn part_two(input: &str) -> impl std::fmt::Display {
    input.replace("abc", "").len()
}
