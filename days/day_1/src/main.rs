fn main() {
    println!("{}", run());
}

fn run() -> i32 {
    include_str!("codes.txt")
        .split("\n")
        .map(|code| {
            let characters = code.chars().into_iter();

            let first_numeric = characters
                .clone()
                .find(|&x| x.is_numeric())
                .expect("Left number not found");

            let last_numeric = characters
                .clone()
                .rev()
                .find(|&x| x.is_numeric())
                .expect("Right number not found");

            format!("{}{}", first_numeric, last_numeric)
                .parse::<i32>()
                .expect("Could not parse string value")
        })
        .sum()
}

#[cfg(test)]
mod day_1 {
    use super::*;

    #[test]
    fn returns_expected_value() {
        assert_eq!(run(), 55029);
    }
}
