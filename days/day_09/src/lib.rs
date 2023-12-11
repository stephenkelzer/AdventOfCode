#[cfg(test)]
mod day_09 {
    use itertools::Itertools;

    #[test]
    fn part_01() {
        let lines = include_str!("input.txt").lines();

        let answer = lines
            .map(|l| {
                l.split_whitespace()
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect_vec()
            })
            .map(|row| {
                let mut layers: Vec<Vec<isize>> = vec![row];
                while !layers.last().unwrap().iter().all(|x| x == &0) {
                    layers.push(
                        layers
                            .last()
                            .unwrap()
                            .windows(2)
                            .filter_map(|w| Some(w[1] - w[0]))
                            .collect_vec(),
                    );
                }

                layers
                    .iter()
                    .rev()
                    .map(|x| x.last().unwrap())
                    .sum::<isize>()
            })
            .sum::<isize>();

        assert_eq!(answer, 1953784198);
    }

    #[test]
    fn part_02() {
        let lines = include_str!("input.txt").lines();

        let answer = lines
            .map(|l| {
                l.split_whitespace()
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect_vec()
            })
            .map(|row| {
                let mut layers: Vec<Vec<isize>> = vec![row];

                while !layers.last().unwrap().iter().all(|x| x == &0) {
                    layers.push(
                        layers
                            .last()
                            .unwrap()
                            .windows(2)
                            .filter_map(|w| Some(w[1] - w[0]))
                            .collect_vec(),
                    );
                }

                layers.iter().rev().skip(2).fold(
                    layers.iter().rev().skip(1).take(1).collect_vec()[0][0],
                    |acc, curr| curr[0] - acc,
                )
            })
            .sum::<isize>();

        assert_eq!(answer, 957);
    }
}
