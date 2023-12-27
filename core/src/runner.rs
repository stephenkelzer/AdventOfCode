use crate::Puzzle;

pub fn runner<F1, F1R, F2, F2R>(puzzle: Puzzle, part_one_func: F1, part_two_func: F2)
where
    F1R: std::fmt::Display,
    F2R: std::fmt::Display,
    F1: Fn(&str) -> F1R,
    F2: Fn(&str) -> F2R,
{
    let input = match std::fs::read_to_string(&puzzle.get_input_file_path()) {
        Ok(file) => {
            assert!(!file.is_empty(), "Input file is empty.");
            file
        }
        Err(e) => {
            println!("{e:?}");
            panic!("Input file not found.");
        }
    };

    let part_one_answer = part_one_func(&input);
    println!("PART 1 ANSWER: {}", part_one_answer);

    let part_two_answer = part_two_func(&input);
    println!("PART 2 ANSWER: {}", part_two_answer);
}

pub fn test<F, FR, E>(puzzle: Puzzle, func: F, expected: E)
where
    FR: std::fmt::Display,
    F: Fn(&str) -> FR,
    E: std::fmt::Display,
{
    let input = match std::fs::read_to_string(&puzzle.get_example_file_path()) {
        Ok(file) => {
            assert!(!file.is_empty(), "Example file is empty.");
            file
        }
        Err(e) => {
            println!("{e:?}");
            panic!("Example file not found.");
        }
    };

    assert_eq!(func(&input).to_string(), expected.to_string());
}
