#[cfg(test)]
mod day_11 {
    use itertools::Itertools;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Point {
        r: usize,
        c: usize,
    }

    #[derive(Debug, PartialEq, Clone)]
    enum Entity {
        Space,
        Galaxy(usize),
    }

    fn solve(expansion_factor: usize) -> usize {
        let mut galaxy_counter = 0;
        let grid = include_str!("input.txt")
            .lines()
            .map(|row| {
                row.chars()
                    .map(|column| match column {
                        '.' => Entity::Space,
                        '#' => {
                            galaxy_counter += 1;
                            Entity::Galaxy(galaxy_counter)
                        }
                        _ => panic!("Unkown char!"),
                    })
                    .collect_vec()
            })
            .collect_vec();

        let expandable_rows = grid
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                if row.iter().any(|x| x != &Entity::Space) {
                    None
                } else {
                    Some(i)
                }
            })
            .collect_vec();

        let expandable_columns = (0..grid[0].len())
            .filter_map(|j| {
                if grid.iter().any(|row| row[j] != Entity::Space) {
                    None
                } else {
                    Some(j)
                }
            })
            .collect_vec();

        let galaxies = grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, column)| matches!(column, Entity::Galaxy(_)))
                    .map(move |(c, _)| Point { r, c })
            })
            .collect_vec();

        galaxies
            .iter()
            .combinations(2)
            .map(|combo| {
                let p1 = combo[0];
                let p2 = combo[1];

                let mut r_factor = expandable_rows
                    .iter()
                    .filter(|x| (p1.r < **x && p2.r >= **x) || (p2.r < **x && p1.r >= **x))
                    .count();

                let mut c_factor = expandable_columns
                    .iter()
                    .filter(|x| (p1.c < **x && p2.c >= **x) || (p2.c < **x && p1.c >= **x))
                    .count();

                if expansion_factor > 1 {
                    r_factor = r_factor * (expansion_factor - 1);
                    c_factor = c_factor * (expansion_factor - 1);
                }

                let row_diff = p1.r.abs_diff(p2.r) + r_factor;
                let col_diff = p1.c.abs_diff(p2.c) + c_factor;
                row_diff + col_diff
            })
            .sum::<usize>()
    }

    #[test]
    fn part_01() {
        assert_eq!(solve(1), 9686930);
    }

    #[test]
    fn part_02() {
        assert_eq!(solve(1000000), 630728425490);
    }
}
