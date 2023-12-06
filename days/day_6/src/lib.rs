#[cfg(test)]
mod day_6 {
    struct Race {
        pub time_ms: usize,
        pub distance_record_mm: usize,
    }

    #[test]
    fn part_one() {
        let races = vec![
            Race {
                time_ms: 61,
                distance_record_mm: 430,
            },
            Race {
                time_ms: 67,
                distance_record_mm: 1036,
            },
            Race {
                time_ms: 75,
                distance_record_mm: 1307,
            },
            Race {
                time_ms: 71,
                distance_record_mm: 1150,
            },
        ];

        let answer = races
            .iter()
            .map(|race| {
                let mut win_count = 0;
                for boat_speed in 1..race.time_ms {
                    let dist = boat_speed * (race.time_ms - boat_speed);

                    if dist > race.distance_record_mm {
                        win_count += 1;
                    }
                }

                win_count
            })
            .product::<usize>();

        assert_eq!(answer, 288);
    }

    #[test]
    fn part_two() {
        assert_eq!(123, 123);
    }
}
