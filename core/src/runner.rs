use crate::Puzzle;

pub fn runner<F1, F1R, F2, F2R>(year: u16, day: u8, part_one_func: F1, part_two_func: F2)
where
    F1R: std::fmt::Display,
    F2R: std::fmt::Display,
    F1: Fn(&str) -> F1R,
    F2: Fn(&str) -> F2R,
{
    let puzzle = Puzzle::new(&year, &day);

    println!("RUNNER FOR: {puzzle:?}");

    let input = &"abcdefghijklmnopqrstuvwxyz".to_string();

    let part_one_answer = part_one_func(input);
    println!("PART 1 ANSWER: {}", part_one_answer);

    let part_two_answer = part_two_func(input);
    println!("PART 2 ANSWER: {}", part_two_answer);
}
