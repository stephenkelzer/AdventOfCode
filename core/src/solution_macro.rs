#[macro_export]
macro_rules! solution {
    ($year:expr, $day:expr, $day_one_func:ident, $day_two_func:ident) => {
        // $crate::solution!(@impl $day, [part_one, 1] [part_two, 2]);
        fn main() {
            // Validate year
            if $year < 1000 || $year > 9999 {
                panic!("Invalid year: {}", $year);
            }

            // Validate day
            if $day < 1 || $day > 31 {
                panic!("Invalid day: {}", $day);
            }

            // Get the function's signature as a string
            let func_signature = stringify!($day_one_func);

            // Validate function signature
            if !func_signature.starts_with("fn")
                || !func_signature.contains("(input: &str)")
                || !func_signature.ends_with("-> usize")
            {
                // compile_error!("Invalid function signature");
            }

            // Call the specified function
            let input: &str = "";
            $day_one_func(input);
            $day_two_func(input);
        }
    }; // ($day:expr, 1) => {
       //     $crate::solution!(@impl $day, [part_one, 1]);
       // };
       // ($day:expr, 2) => {
       //     $crate::solution!(@impl $day, [part_two, 2]);
       // };

       // (@impl $day:expr, $( [$func:expr, $part:expr] )*) => {
       //     /// The current day.
       //     // const DAY: $crate::template::Day = $crate::day!($day);

       //     fn main() {
       //         use $crate::template::runner::*;

       //         let input = $crate::template::read_file("inputs", DAY);

       //         $( run_part($func, &input, DAY, $part); )*
       //     }
       // };
}
