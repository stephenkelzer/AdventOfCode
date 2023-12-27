#[macro_export]
macro_rules! solution {
    ($year:expr, $day:expr, $day_one_func:ident, $day_two_func:ident) => {
        fn main() {
            // Validate year
            if $year < 2015 || $year > core::config::get_latest_year() {
                panic!("Invalid year: {}", $year);
            }

            // Validate day
            if $day < 1 || $day > 25 {
                panic!("Invalid day: {}", $day);
            }

            core::runner::runner($year, $day, $day_one_func, $day_two_func);
        }
    };
}
