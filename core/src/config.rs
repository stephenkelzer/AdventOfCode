use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub session_token: String,
    pub default_year: u16,
    pub latest_year: u16,
}

impl Configuration {
    pub fn new() -> Self {
        dotenv().ok();

        let latest_year = 2023;
        let default_year = env::var("AOC_DEFAULT_YEAR")
            .expect("Could not find `AOC_DEFAULT_YEAR` environment variable in `.env` file.")
            .parse::<u16>()
            .expect(
                "`AOC_DEFAULT_YEAR` wasn't formatted in the expected format of u16 (e.g. '2023').",
            );

        let allowed_year_range = 2015..=latest_year;
        if !allowed_year_range.contains(&default_year) {
            panic!("`{default_year}` is out of the allowed range: {allowed_year_range:?}. Check your `.env` file and ensure that the `AOC_DEFAULT_YEAR` value is within the allowed range.");
        }

        Self {
            session_token: env::var("AOC_SESSION_TOKEN")
                .expect("Could not find `AOC_SESSION_TOKEN` environment variable in `.env` file."),
            default_year,
            latest_year,
        }
    }
}
