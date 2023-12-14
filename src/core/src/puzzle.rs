use std::path::Path;

pub struct Puzzle {
    year: u16,
    day: u8,
}

impl Puzzle {
    pub const fn new(year: u16, day: u8) -> Self {
        Puzzle { year, day }
    }

    pub fn get_year_value(&self) -> u16 {
        self.year
    }

    pub fn get_year_display(&self) -> String {
        format!("{}", self.year)
    }

    pub fn get_day_value(&self) -> u8 {
        self.day
    }

    pub fn get_day_display(&self) -> String {
        format!("{:02}", self.day)
    }

    pub fn get_crate_name(&self) -> String {
        format!(
            "challenge_{}_{}",
            self.get_year_display(),
            self.get_day_display()
        )
    }

    pub fn get_crate_directory(&self) -> String {
        format!(
            "./src/challenges/{}/{}",
            self.get_year_display(),
            self.get_day_display()
        )
    }

    pub fn is_scaffolded(&self) -> bool {
        let crate_manifest_file = &format!("{}/Cargo.toml", self.get_crate_directory());
        println!("{:?}", crate_manifest_file);
        Path::new(&crate_manifest_file).exists()
    }
}

#[macro_export]
macro_rules! puzzle {
    ($year:expr, $day:expr) => {
        $crate::puzzle!(@impl $year, @impl $day, [part_one, 1] [part_two, 2]);
    };
    ($year:expr, $day:expr, 1) => {
        $crate::puzzle!(@impl $year, @impl $day, [part_one, 1]);
    };
    ($year:expr, $day:expr, 2) => {
        $crate::puzzle!(@impl $year, @impl $day, [part_two, 2]);
    };

    (@impl $year:expr, @impl $day:expr, $( [$func:expr, $part:expr] )*) => {
        const _ASSERT_YEAR: () = assert!(
            $year >= 2015 && $year <= 2023,
            concat!(
                "invalid year number `",
                $year,
                "`, expecting a value between 2015 and 2023"
            ),
        );
        const _ASSERT_DAY: () = assert!(
            $day > 0 && $day <= 25,
            concat!(
                "invalid day number `",
                $day,
                "`, expecting a value between 1 and 25"
            ),
        );

        const PUZZLE: $crate::puzzle::Puzzle = $crate::puzzle::Puzzle::new($year, $day);

        fn main() {
            use $crate::runner::run_part;
            // let input = $crate::template::read_file("inputs", DAY);
            let input = include_str!("input.txt");
            $( run_part($func, input, PUZZLE, $part); )*
            println!("YAY!");
        }
    };
}
