#[cfg(test)]
mod day_14 {
    use itertools::Itertools;

    #[test]
    fn part_01() {
        let mut grid = include_str!("input.txt")
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();

        for r in 1..grid.len() {
            for c in 0..grid[r].len() {
                let col = grid[r][c];
                println!("{:?}|{:?} => {:?}", r, c, col);
                if col != 'O' {
                    continue;
                }

                let mut high_r: Option<usize> = None;
                let mut curr_r = r.clone() - 1;

                loop {
                    println!("checking: {}|{}", curr_r, c);

                    if grid[curr_r][c] != '.' {
                        break;
                    }

                    high_r = Some(curr_r);

                    if curr_r == 0 {
                        break;
                    }

                    curr_r -= 1;
                }

                if let Some(high_r) = high_r {
                    // slide the rock up
                    println!("MOVING: {}", high_r);
                    grid[high_r][c] = 'O';
                    grid[r][c] = '.';
                }
            }
        }

        let grid_height = grid.len();

        let answer = grid
            .iter()
            .enumerate()
            .map(|(r, row)| {
                let rock_count = row.iter().filter(|x| **x == 'O').count();
                (grid_height - r) * rock_count
            })
            .sum::<usize>();

        assert_eq!(answer, 106990);
    }

    #[test]
    fn part_02() {
        let grids = include_str!("input.txt");

        assert_eq!(123, 123);
    }
}
