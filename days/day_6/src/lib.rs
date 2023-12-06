#[cfg(test)]
mod day_6 {
    struct Race(pub usize, pub usize);

    #[test]
    fn part_one() {
        let races = vec![
            Race(61, 430),
            Race(67, 1036),
            Race(75, 1307),
            Race(71, 1150),
        ];

        let answer = races
            .iter()
            .map(|race| {
                let mut win_count = 0;
                for boat_mm_per_ms in 1..race.0 {
                    let distance_travelled = boat_mm_per_ms * (race.0 - boat_mm_per_ms);

                    if distance_travelled > race.1 {
                        win_count += 1;
                    }
                }

                win_count
            })
            .product::<usize>();

        assert_eq!(answer, 316800);
    }

    #[test]
    fn part_two() {
        let race = Race(61677571, 430103613071150);

        let mut win_count = 0;
        for boat_mm_per_ms in 1..race.0 {
            let distance_travelled = boat_mm_per_ms * (race.0 - boat_mm_per_ms);

            if distance_travelled > race.1 {
                win_count += 1;
            }
        }

        assert_eq!(win_count, 45647654);
    }
}
