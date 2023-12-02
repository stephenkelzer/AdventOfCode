#[cfg(test)]
mod day_1 {
    #[test]
    fn part_one() {
        let result: usize = include_str!("codes.txt")
            .split("\n")
            .map(|code| {
                let numeric_characters =
                    code.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();

                let first_numeric = numeric_characters.first().expect("Left number not found");

                let last_numeric = numeric_characters.last().expect("Right number not found");

                format!("{}{}", first_numeric, last_numeric)
                    .parse::<usize>()
                    .expect("Could not parse string value")
            })
            .sum();

        assert_eq!(result, 55029);
    }

    #[test]
    fn part_two() {
        let result: usize = include_str!("codes.txt")
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
                let numeric_characters =
                    code.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();

                let first_numeric = numeric_characters.first().expect("Left number not found");

                let last_numeric = numeric_characters.last().expect("Right number not found");

                format!("{}{}", first_numeric, last_numeric)
                    .parse::<usize>()
                    .expect("Could not parse string value")
            })
            .sum();

        assert_eq!(result, 55686);
    }
}
