use crate::puzzle::Puzzle;

pub fn run_part(_func: impl Fn() -> usize, _input: &str, puzzle: Puzzle, part: u8) {
    let part_str = format!("Part {part}");

    // println!("{:?}", input);

    println!(
        "{}|{}|{}",
        puzzle.get_year_display(),
        puzzle.get_day_display(),
        part_str
    )

    // let (result, duration, samples) =
    //     run_timed(func, input, |result| print_result(result, &part_str, ""));

    // print_result(&result, &part_str, &format_duration(&duration, samples));

    // if let Some(result) = result {
    //     submit_result(result, day, part);
    // }
}
