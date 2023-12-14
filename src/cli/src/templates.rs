pub const MODULE_TEMPLATE: &str = r#"
core::solution!(%YEAR_NUMBER%, %DAY_NUMBER%)

pub fn part_one(input: &str) -> isize {
    0
}

pub fn part_two(input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
"#;

pub const CARGO_TOML_TEMPLATE: &str = r#"
[package]
name = "%CRATE_NAME%"
version.workspace = true
edition.workspace = true
authors.workspace = true

[lib]
doctest = false

[dependencies]
core.workspace = true
"#;
