#[macro_export]
macro_rules! solver {
    ($year:expr, $day: expr, $day_one_func: ident, $day_two_func: ident, $day_one_expected: expr, $day_two_expected: expr) => {
        fn main() {
            let puzzle = core::Puzzle::new(&$year, &$day);
            core::runner::runner(puzzle, $day_one_func, $day_two_func);
        }

        // TODO: rename "tests" module name to something like: tests_2023_02 or 2023_02_tests
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_part_one() {
                let puzzle = core::Puzzle::new(&$year, &$day);
                core::runner::test(puzzle, $day_one_func, $day_one_expected);
            }

            #[test]
            fn test_part_two() {
                let puzzle = core::Puzzle::new(&$year, &$day);
                core::runner::test(puzzle, $day_two_func, $day_two_expected);
            }
        }
    };
}
