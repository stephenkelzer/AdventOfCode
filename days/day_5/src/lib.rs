#[cfg(test)]
mod day_5 {
    use itertools::Itertools;

    #[test]
    fn part_one() {
        let mut lines = include_str!("input.txt").lines();

        let mut seeds = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|n| (n.parse::<usize>().unwrap(), false))
            .collect_vec();

        for line in lines {
            if !line.is_empty() && !line.contains("map") {
                let records = line
                    .trim()
                    .split(' ')
                    .map(|x| x.parse::<usize>().expect("failed to parse record number"))
                    .collect::<Vec<_>>();

                let destination_range_start = records[0];
                let source_range_start = records[1];
                let range_length = records[2];

                seeds = seeds
                    .iter()
                    .map(|(seed, seen)| {
                        if !seen
                            && *seed >= source_range_start
                            && *seed <= (source_range_start + range_length)
                        {
                            (destination_range_start + (*seed - source_range_start), true)
                        } else {
                            (*seed, *seen)
                        }
                    })
                    .collect();
            } else {
                seeds = seeds.iter().map(|(seed, _)| (*seed, false)).collect()
            }
        }

        let result = seeds.iter().map(|(seed, _)| seed).min().unwrap();

        assert_eq!(*result, 227653707);
    }

    #[test]
    fn part_two() {
        let mut lines = include_str!("input.txt").lines();

        let seed_ranges = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec()
            .chunks(2)
            .map(|c| c[0]..(c[0] + c[1]))
            .collect_vec();

        let mut maps: Vec<Vec<(usize, usize, usize)>> = vec![];
        let mut temp_map: Option<Vec<(usize, usize, usize)>> = None;

        lines.skip(1).for_each(|line| {
            match line.trim() {
                line if line.ends_with("map:") => {
                    // NEW MAP
                    temp_map = Some(vec![]);
                }
                line if line.is_empty() => {
                    // END OF MAP
                    maps.push(temp_map.as_mut().expect("No current map!").clone());
                    temp_map = None;
                }
                line => {
                    // VALUE IN CURRENT MAP
                    let records = line
                        .trim()
                        .split(' ')
                        .map(|x| x.parse::<usize>().expect("failed to parse record number"))
                        .collect_vec();

                    temp_map
                        .as_mut()
                        .expect("No current map!")
                        .push((records[0], records[1], records[2]));
                }
            };
        });

        if let Some(current_map) = temp_map {
            // catch hanging map at end of file
            maps.push(current_map);
        }

        let in_range = |seed: usize| seed_ranges.iter().any(|x| x.start <= seed && x.end >= seed);

        let get_seed_location = |mut step: usize| -> usize {
            for map in maps.iter().rev() {
                for (destination_range_start, source_range_start, range_length) in map {
                    if destination_range_start <= &step
                        && destination_range_start + range_length > step
                    {
                        step = source_range_start + step - destination_range_start;
                        break;
                    }
                }
            }

            step
        };

        let mut answer: Option<usize> = None;
        let mut index: usize = 0;
        while answer.is_none() {
            let location = get_seed_location(index);
            if in_range(location) {
                answer = Some(index);
            }

            index += 1;
        }

        assert_eq!(answer.unwrap(), 78775051);
    }
}
