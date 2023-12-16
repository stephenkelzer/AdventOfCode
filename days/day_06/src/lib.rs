#[cfg(test)]
mod day_06 {
    #[test]
    fn part_01() {
        let mut lines = include_str!("input.txt").lines();
        let times = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let distances = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let answer = times
            .iter()
            .zip(distances)
            .map(|(race_time, record_distance)| {
                let mut win_count = 0;
                for boat_mm_per_ms in 1..*race_time {
                    let distance_travelled = boat_mm_per_ms * (race_time - boat_mm_per_ms);

                    if distance_travelled > record_distance {
                        win_count += 1;
                    }
                }

                win_count
            })
            .product::<usize>();

        assert_eq!(answer, 316800);
    }

    #[test]
    fn part_02() {
        let mut lines = include_str!("input.txt").lines();
        let race_time = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .replace(' ', "")
            .parse::<usize>()
            .unwrap();
        let record_distance = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .replace(' ', "")
            .parse::<usize>()
            .unwrap();

        let mut win_count = 0;
        for boat_mm_per_ms in 1..race_time {
            let distance_travelled = boat_mm_per_ms * (race_time - boat_mm_per_ms);

            if distance_travelled > record_distance {
                win_count += 1;
            }
        }

        assert_eq!(win_count, 45647654);
    }
}
