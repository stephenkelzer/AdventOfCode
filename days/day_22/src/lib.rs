#[cfg(test)]
mod day_22 {
    use itertools::{iproduct, Itertools};
    use std::cmp::{max, Ordering};

    fn bounds(bricks: &Vec<((usize, usize, usize), (usize, usize, usize))>) -> (usize, usize) {
        let (mut x_max, mut y_max) = (0, 0);
        for &(_, (x2, y2, _)) in bricks {
            (x_max, y_max) = (max(x_max, x2), max(y_max, y2));
        }
        (x_max, y_max)
    }

    #[test]
    fn part_01() {
        let bricks = include_str!("input.txt")
            .lines()
            .map(|l| {
                let (x1, y1, z1, x2, y2, z2) = l
                    .split_terminator(&[',', '~'])
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                ((x1, y1, z1), (x2, y2, z2))
            })
            .sorted_by_key(|&((x, y, z), _)| (z, x, y))
            .collect_vec();

        let (x_max, y_max) = bounds(&bricks);

        let mut supporting = vec![false; bricks.len()];
        let mut last = vec![(usize::MAX, 0); (x_max + 1) * (y_max + 1)];

        for (i, &((x1, y1, z1), (x2, y2, z2))) in bricks.iter().enumerate() {
            let z_target = iproduct!(y1..y2 + 1, x1..x2 + 1)
                .map(|(y, x)| last[y * (x_max + 1) + x].1)
                .max()
                .unwrap_or(0);

            let mut supported_by = None;
            let mut supporting_count = 0;

            for y in y1..y2 + 1 {
                for x in x1..x2 + 1 {
                    let (s, l) = last[y * (x_max + 1) + x];
                    if l == z_target && s != usize::MAX {
                        if supported_by != Some(s) {
                            supporting_count += 1;
                            supported_by = Some(s);
                        }
                    }
                    last[y * (x_max + 1) + x] = (i, z_target + z2 - z1 + 1);
                }
            }

            if supporting_count == 1 {
                supporting[supported_by.unwrap()] = true;
            }
        }

        let answer = supporting.iter().filter(|&&sup| !sup).count();

        assert_eq!(answer, 495);
    }

    #[test]
    fn part_02() {
        let bricks = include_str!("input.txt")
            .lines()
            .map(|l| {
                let (x1, y1, z1, x2, y2, z2) = l
                    .split_terminator(&[',', '~'])
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                ((x1, y1, z1), (x2, y2, z2))
            })
            .sorted_by_key(|&((x, y, z), _)| (z, x, y))
            .collect_vec();

        let (x_max, y_max) = bounds(&bricks);
        let mut last = vec![(usize::MAX, 0); (x_max + 1) * (y_max + 1)];
        let mut under = Vec::with_capacity(4);
        let mut i_dominators = vec![0; bricks.len() + 1];
        let mut n_dominators = vec![0; bricks.len() + 1];

        let mut sum = 0;

        for (i, &((x1, y1, z1), (x2, y2, z2))) in bricks.iter().enumerate() {
            let z_target = iproduct!(y1..y2 + 1, x1..x2 + 1)
                .map(|(y, x)| last[y * (x_max + 1) + x].1)
                .max()
                .unwrap_or(0);

            under.clear();
            for y in y1..y2 + 1 {
                for x in x1..x2 + 1 {
                    let (s, l) = last[y * (x_max + 1) + x];
                    if l == z_target && s != usize::MAX {
                        if !under.contains(&s) {
                            under.push(s);
                        }
                    }
                    last[y * (x_max + 1) + x] = (i, z_target + z2 - z1 + 1);
                }
            }

            i_dominators[i + 1] = under
                .iter()
                .map(|&n| n + 1)
                .reduce(|mut f1, mut f2| loop {
                    match f1.cmp(&f2) {
                        Ordering::Less => f2 = i_dominators[f2],
                        Ordering::Greater => f1 = i_dominators[f1],
                        Ordering::Equal => return f1,
                    }
                })
                .unwrap_or(0);
            n_dominators[i + 1] = n_dominators[i_dominators[i + 1]] + 1;
            sum += n_dominators[i + 1];
        }

        let answer = sum - bricks.len();

        assert_eq!(answer, 76158);
    }
}
