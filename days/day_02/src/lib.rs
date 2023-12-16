#[cfg(test)]
mod day_02 {
    #[test]
    fn part_01() {
        const MAX_RED: usize = 12;
        const MAX_GREEN: usize = 13;
        const MAX_BLUE: usize = 14;

        let result: usize = include_str!("input.txt")
            .split('\n')
            .filter_map(|line| {
                let splits = line.split(':').collect::<Vec<_>>();

                let game_id = splits
                    .first()
                    .expect("Couldn't parse game identifier string")
                    .trim()
                    .replace("Game ", "")
                    .parse::<usize>()
                    .expect("Couldn't parse game id");

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

                    round.split(',').map(|x| x.trim()).for_each(|pull| {
                        let values = pull.split(' ').collect::<Vec<_>>();
                        let qty = values
                            .first()
                            .expect("Could not parse qty")
                            .parse::<usize>()
                            .expect("Could not convert qty to usize");

                        match *values.last().expect("Could not parse color") {
                            "red" => {
                                if qty > red_seen {
                                    red_seen = qty;
                                }
                            }
                            "blue" => {
                                if qty > blue_seen {
                                    blue_seen = qty;
                                }
                            }
                            "green" => {
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
            .sum();

        assert_eq!(result, 2156);
    }

    #[test]
    fn part_02() {
        let result: usize = include_str!("input.txt")
            .split('\n')
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
                        round.split(',').map(|x| x.trim()).for_each(|pull| {
                            let values = pull.split(' ').collect::<Vec<_>>();
                            let qty = values
                                .first()
                                .expect("Could not parse qty")
                                .parse::<usize>()
                                .expect("Could not convert qty to usize");

                            match *values.last().expect("Could not parse color") {
                                "red" => {
                                    if qty > max_red_seen {
                                        max_red_seen = qty;
                                    }
                                }
                                "blue" => {
                                    if qty > max_blue_seen {
                                        max_blue_seen = qty;
                                    }
                                }
                                "green" => {
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
            .sum();

        assert_eq!(result, 66909);
    }
}
