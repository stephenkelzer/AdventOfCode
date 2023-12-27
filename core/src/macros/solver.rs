#[macro_export]
macro_rules! solver {
    ($year:expr, $day: expr, $day_one_func: ident, $day_two_func: ident, $day_one_expected: expr, $day_two_expected: expr) => {
        fn main() {
            let puzzle = core::Puzzle::new(&$year, &$day);
            core::runner::runner(puzzle, $day_one_func, $day_two_func);
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn part_one() {
                let puzzle = core::Puzzle::new(&$year, &$day);
                core::runner::test(puzzle, $day_one_func, $day_one_expected);
            }

            #[test]
            fn part_two() {
                let puzzle = core::Puzzle::new(&$year, &$day);
                core::runner::test(puzzle, $day_two_func, $day_two_expected);
            }
        }
    };
}
