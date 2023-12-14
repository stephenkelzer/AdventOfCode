#[cfg(test)]
mod day_13 {
    use itertools::Itertools;
    use std::{
        collections::{hash_map::DefaultHasher, HashMap},
        hash::{Hash, Hasher},
    };

    fn north(grid: &mut Vec<Vec<char>>) {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 'O' {
                    let mut k = i;
                    while k > 0 && grid[k - 1][j] == '.' {
                        grid[k][j] = '.';
                        grid[k - 1][j] = 'O';
                        k -= 1;
                    }
                }
            }
        }
    }

    fn west(grid: &mut Vec<Vec<char>>) {
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 'O' {
                    let mut k = j;
                    while k > 0 && grid[i][k - 1] == '.' {
                        grid[i][k] = '.';
                        grid[i][k - 1] = 'O';
                        k -= 1;
                    }
                }
            }
        }
    }

    fn south(grid: &mut Vec<Vec<char>>) {
        for i in (0..grid.len()).rev() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 'O' {
                    let mut k = i;
                    while k < grid.len() - 1 && grid[k + 1][j] == '.' {
                        grid[k][j] = '.';
                        grid[k + 1][j] = 'O';
                        k += 1;
                    }
                }
            }
        }
    }

    fn east(grid: &mut Vec<Vec<char>>) {
        for i in 0..grid.len() {
            for j in (0..grid[i].len()).rev() {
                if grid[i][j] == 'O' {
                    let mut k = j;
                    while k < grid[i].len() - 1 && grid[i][k + 1] == '.' {
                        grid[i][k] = '.';
                        grid[i][k + 1] = 'O';
                        k += 1;
                    }
                }
            }
        }
    }

    fn calculate_load(grid: &Vec<Vec<char>>) -> usize {
        let mut load = 0;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 'O' {
                    load += grid.len() - i;
                }
            }
        }

        load
    }

    fn hash(grid: &Vec<Vec<char>>) -> u64 {
        let mut hasher = DefaultHasher::new();
        Hash::hash_slice(&grid, &mut hasher);
        hasher.finish()
    }

    #[test]
    fn part_01() {
        let mut grid = include_str!("input.txt")
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();

        north(&mut grid);

        let answer = calculate_load(&grid);

        assert_eq!(answer, 106990);
    }

    #[test]
    fn part_02() {
        let mut grid = include_str!("input.txt")
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();

        let mut cache: HashMap<u64, usize> = HashMap::new();
        let mut cycle_found_at = 0;
        let mut answer: usize = 0;

        for j in 0..1000 {
            north(&mut grid);
            west(&mut grid);
            south(&mut grid);
            east(&mut grid);

            let hash = hash(&grid);

            if cache.contains_key(&hash) {
                if cycle_found_at == 0 {
                    cycle_found_at = j;
                    cache.clear();
                    cache.insert(hash, j);
                    continue;
                }

                let mut remaining = 1_000_000_000 - j - 1;

                remaining %= j - cycle_found_at;

                for _ in 0..remaining {
                    north(&mut grid);
                    west(&mut grid);
                    south(&mut grid);
                    east(&mut grid);
                }

                answer = calculate_load(&grid);
                break;
            }

            cache.insert(hash, j);
        }

        assert_eq!(answer, 100531);
    }
}
