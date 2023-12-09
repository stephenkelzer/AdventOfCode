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

            let mut x_factor = layers.iter().rev().skip(1).take(1).collect_vec()[0][0];

            for layer in layers.iter().rev().skip(2) {
                x_factor = layer[0] - x_factor;
            }

            results.push(x_factor);
        }

        assert_eq!(results.iter().sum::<isize>(), 957);
    }
}
