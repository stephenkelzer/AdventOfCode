#[macro_export]
macro_rules! solver {
    ($year:expr, $day:expr, $day_one_func:ident, $day_two_func:ident) => {
        fn main() {
            let puzzle = core::Puzzle::new(&$year, &$day);
            core::runner::runner(puzzle, $day_one_func, $day_two_func);
        }
    };
}
