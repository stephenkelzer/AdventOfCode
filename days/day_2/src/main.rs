fn main() {
    println!("part_one: {}", run_part_one());
    println!("part_two: {}", run_part_two());
}

fn run_part_one() -> usize {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    include_str!("games.txt")
        .split("\n")
        .filter_map(|line| {
            // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            let splits = line.split(':').collect::<Vec<_>>();

            // example: 1
            let game_id = splits
                .first()
                .expect("Couldn't parse game identifier string")
                .trim()
                .replace(&"Game ", &"")
                .parse::<usize>()
                .expect("Couldn't parse game id");

            // example: ["5 blue, 6 red", "8 red", "1 green, 9 blue, 5 red"]
            let rounds = splits
                .last()
                .expect("Couldn't parse game values string")
                .trim()
                .split(';')
                .map(|x| x.trim())
                .collect::<Vec<_>>();

            let valid = rounds.into_iter().all(|round| {
                let mut red_seen: usize = 0;
                let mut green_seen: usize = 0;
                let mut blue_seen: usize = 0;

                round.split(",").map(|x| x.trim()).for_each(|pull| {
                    let values = pull.split(" ").collect::<Vec<_>>();
                    let qty = values
                        .first()
                        .expect("Could not parse qty")
                        .parse::<usize>()
                        .expect("Could not convert qty to usize");
                    let color = values.last().expect("Could not parse color");

                    match color {
                        &"red" => {
                            if qty > red_seen {
                                red_seen = qty;
                            }
                        }
                        &"blue" => {
                            if qty > blue_seen {
                                blue_seen = qty;
                            }
                        }
                        &"green" => {
                            if qty > green_seen {
                                green_seen = qty;
                            }
                        }
                        _ => {
                            panic!("Unknown color encountered")
                        }
                    }
                });

                red_seen <= MAX_RED && green_seen <= MAX_GREEN && blue_seen <= MAX_BLUE
            });

            if valid {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

fn run_part_two() -> usize {
    include_str!("games.txt")
        .split("\n")
        .map(|line| {
            let mut max_red_seen: usize = 0;
            let mut max_green_seen: usize = 0;
            let mut max_blue_seen: usize = 0;

            line.split(':')
                .collect::<Vec<_>>()
                .last()
                .expect("Couldn't parse game values string")
                .trim()
                .split(';')
                .map(|x| x.trim())
                .for_each(|round| {
                    round.split(",").map(|x| x.trim()).for_each(|pull| {
                        let values = pull.split(" ").collect::<Vec<_>>();
                        let qty = values
                            .first()
                            .expect("Could not parse qty")
                            .parse::<usize>()
                            .expect("Could not convert qty to usize");
                        let color = values.last().expect("Could not parse color");

                        match color {
                            &"red" => {
                                if qty > max_red_seen {
                                    max_red_seen = qty;
                                }
                            }
                            &"blue" => {
                                if qty > max_blue_seen {
                                    max_blue_seen = qty;
                                }
                            }
                            &"green" => {
                                if qty > max_green_seen {
                                    max_green_seen = qty;
                                }
                            }
                            _ => {
                                panic!("Unknown color encountered")
                            }
                        }
                    });
                });

            max_red_seen * max_green_seen * max_blue_seen
        })
        .sum::<usize>()
}

#[cfg(test)]
mod day_2 {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(run_part_one(), 2156);
    }

    #[test]
    fn part_two() {
        assert_eq!(run_part_two(), 66909);
    }
}
