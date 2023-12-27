use chrono::Datelike;
use dotenvy::dotenv;
use std::env;

const SESSION_TOKEN_KEY: &str = "AOC_SESSION_TOKEN";
const DEFAULT_YEAR_KEY: &str = "AOC_DEFAULT_YEAR_OVERRIDE";

pub fn get_session_token() -> String {
    dotenv().ok();

    env::var(SESSION_TOKEN_KEY).expect(&format!(
        "Could not find `{SESSION_TOKEN_KEY}` environment variable in `.env` file."
    ))
}

pub fn get_latest_year() -> u16 {
    dotenv().ok();

    let now = chrono::offset::Utc::now();
    let curr_year: u16 = now
        .year()
        .try_into()
        .expect("Couldn't parse curr_year into u16");

    match now.month() {
        12 => curr_year,
        _ => curr_year - 1,
    }
}

pub fn get_default_year() -> u16 {
    dotenv().ok();

    let latest_year = get_latest_year();

    let default_year: u16 = match env::var(DEFAULT_YEAR_KEY) {
        Ok(val) => val.parse::<u16>().expect(&format!(
            "`{DEFAULT_YEAR_KEY}` wasn't formatted in the expected format of u16 (e.g. '{latest_year}')."
        )),
        Err(_) => latest_year,
    };

    let allowed_year_range = 2015..=latest_year;
    if !allowed_year_range.contains(&default_year) {
        panic!("`{default_year}` is out of the allowed range: {allowed_year_range:?}. Check your `.env` file and ensure that the `{DEFAULT_YEAR_KEY}` value is within the allowed range.");
    }

    default_year
}
