#[cfg(test)]
mod day_9 {
    use itertools::Itertools;

    #[test]
    fn part_one() {
        let lines = include_str!("input.txt").lines();
        let list = lines
            .map(|l| {
                l.split_whitespace()
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let dive = |input: Vec<isize>| -> Vec<isize> {
            input
                .windows(2)
                .filter_map(|w| Some(w[1] - w[0]))
                .collect_vec()
        };

        let mut results = vec![];

        for row in list {
            let mut layers: Vec<Vec<isize>> = vec![row];
            while !layers.last().unwrap().iter().all(|x| x == &0) {
                layers.push(dive(layers.last().unwrap().to_vec()));
            }

            results.push(
                layers
                    .iter()
                    .rev()
                    .map(|x| x.last().unwrap())
                    .sum::<isize>(),
            )
        }

        assert_eq!(results.iter().sum::<isize>(), 1953784198);
    }

    #[test]
    fn part_two() {
        // let mut lines = include_str!("input.txt").lines();

        assert_eq!(123, 123);
    }
}
