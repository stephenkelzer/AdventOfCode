fn main() {
    println!("part_one: {}", run_part_one());
    println!("part_two: {}", run_part_two());
}

fn run_part_one() -> i32 {
    123
}

fn run_part_two() -> i32 {
    123
}

#[cfg(test)]
mod day_3 {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(run_part_one(), 123);
    }

    #[test]
    fn part_two() {
        assert_eq!(run_part_two(), 123);
    }
}
