core::puzzle!(2023, 1);

fn part_one() -> usize {
    let result: usize = include_str!("input.txt")
        .split("\n")
        .map(|code| {
            let numeric_characters = code.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();

            let first_numeric = numeric_characters.first().expect("Left number not found");

            let last_numeric = numeric_characters.last().expect("Right number not found");

            format!("{}{}", first_numeric, last_numeric)
                .parse::<usize>()
                .expect("Could not parse string value")
        })
        .sum();

    result
}

fn part_two() -> usize {
    let result: usize = include_str!("input.txt")
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
        .split("\n")
        .map(|code| {
            let numeric_characters = code.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();

            let first_numeric = numeric_characters.first().expect("Left number not found");

            let last_numeric = numeric_characters.last().expect("Right number not found");

            format!("{}{}", first_numeric, last_numeric)
                .parse::<usize>()
                .expect("Could not parse string value")
        })
        .sum();

    result
}
