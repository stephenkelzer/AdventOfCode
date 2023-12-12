#[cfg(test)]
mod day_12 {
    use itertools::Itertools;
    use std::collections::HashMap;

    fn arrangements(
        cache: &mut HashMap<(usize, usize, usize), usize>,
        slice: &[char],
        within: Option<usize>,
        map: &[usize],
    ) -> usize {
        if slice.is_empty() {
            return match (within, map.len()) {
                (None, 0) => 1,
                (Some(x), 1) if x == map[0] => 1,
                _ => 0,
            };
        }

        if within.is_some() && map.is_empty() {
            return 0;
        }

        let key = (slice.len(), within.unwrap_or(0), map.len());
        if let Some(&x) = cache.get(&key) {
            return x;
        }

        let ways = match (slice[0], within) {
            ('.', Some(x)) if x != map[0] => 0,
            ('.', Some(_)) => arrangements(cache, &slice[1..], None, &map[1..]),
            ('.', None) => arrangements(cache, &slice[1..], None, map),
            ('#', Some(_)) => arrangements(cache, &slice[1..], within.map(|x| x + 1), map),
            ('#', None) => arrangements(cache, &slice[1..], Some(1), map),
            ('?', Some(x)) => {
                let mut result = arrangements(cache, &slice[1..], within.map(|x| x + 1), map);
                if x == map[0] {
                    result += arrangements(cache, &slice[1..], None, &map[1..])
                }
                result
            }
            ('?', None) => {
                arrangements(cache, &slice[1..], Some(1), map)
                    + arrangements(cache, &slice[1..], None, map)
            }
            _ => panic!("Shouldn't get here!"),
        };

        cache.insert(key, ways);

        ways
    }

    #[test]
    fn part_01() {
        let answer = include_str!("input.txt")
            .lines()
            .map(|l| {
                let (springs, map) = l.trim().split_once(" ").unwrap();
                let map = map
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_vec();

                (springs.trim(), map)
            })
            .map(|(springs, map)| {
                let mut cache = HashMap::new();
                arrangements(&mut cache, &springs.chars().collect_vec()[..], None, &map)
            })
            .sum::<usize>();

        assert_eq!(answer, 7857);
    }

    #[test]
    fn part_02() {
        let mut cache = HashMap::new();
        let answer = include_str!("input.txt")
            .lines()
            .map(|l| {
                let (springs, map) = l.trim().split_once(" ").unwrap();
                let map = map
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect_vec();

                (springs.trim(), map)
            })
            .map(|(springs, map)| {
                let springs = (0..5).map(|_| springs).join("?");
                let map = (0..5).flat_map(|_| &map).copied().collect_vec();

                cache.clear();

                arrangements(&mut cache, &springs.chars().collect_vec()[..], None, &map)
            })
            .sum::<usize>();

        assert_eq!(answer, 28606137449920);
    }
}
