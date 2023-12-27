use crate::Puzzle;

pub fn runner<F1, F1R, F2, F2R>(puzzle: Puzzle, part_one_func: F1, part_two_func: F2)
where
    F1R: std::fmt::Display,
    F2R: std::fmt::Display,
    F1: Fn(&str) -> F1R,
    F2: Fn(&str) -> F2R,
{
    println!("RUNNER FOR: {puzzle:?}");

    let input = std::fs::read_to_string(&puzzle.get_input_file_path())
        .expect("input file could not be read.");
    if input.is_empty() {
        panic!("Input is empty!");
    }

    let part_one_answer = part_one_func(&input);
    println!("PART 1 ANSWER: {}", part_one_answer);

    let part_two_answer = part_two_func(&input);
    println!("PART 2 ANSWER: {}", part_two_answer);
}

pub fn test<F, FR, E>(func: F, example_input: &str, expected: E)
where
    FR: std::fmt::Display,
    F: Fn(&str) -> FR,
    E: std::fmt::Display,
{
    let cleaned_example_input: String = example_input
        .trim_start_matches("\n")
        .trim_end_matches("\n")
        .lines()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join("\n");

    assert!(!cleaned_example_input.is_empty(), "Example input is empty.");
    assert_eq!(
        func(&cleaned_example_input).to_string(),
        expected.to_string()
    );
}
