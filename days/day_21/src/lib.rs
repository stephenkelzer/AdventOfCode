#[cfg(test)]
mod day_21 {
    use itertools::Itertools;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn part_01() {
        fn get_neighbors(grid: &Vec<Vec<char>>, row: &usize, col: &usize) -> Vec<(usize, usize)> {
            let neighbor_dirs: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut neighbors = Vec::new();
            let height = grid.len();
            let width = grid[0].len();

            for (dr, dc) in neighbor_dirs {
                let r = *row as isize + dr;
                let c = *col as isize + dc;
                if r >= 0 && r < height as isize && c >= 0 && c < width as isize {
                    let ch = grid[r as usize][c as usize];
                    if ch == '.' || ch == 'S' {
                        neighbors.push((r as usize, c as usize));
                    }
                }
            }

            neighbors
        }

        let mut steps_left = 64;
        let mut positions: HashSet<(usize, usize)> = HashSet::new();
        let mut cache: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

        let grid = include_str!("input.txt")
            .lines()
            .enumerate()
            .map(|(ri, r)| {
                r.chars()
                    .enumerate()
                    .map(|(ci, c)| {
                        if c == 'S' {
                            positions.insert((ri, ci));
                        }

                        c
                    })
                    .collect_vec()
            })
            .collect_vec();

        while steps_left > 0 {
            let poses = positions.iter().cloned().collect_vec();
            positions.clear();

            for (r, c) in poses {
                if let Some(neighbors) = cache.get_mut(&(r, c)) {
                    positions.extend(neighbors.clone());
                    continue;
                }
                let neighbors = get_neighbors(&grid, &r, &c);
                positions.extend(neighbors.clone());
                cache.insert((r, c), neighbors);
            }

            steps_left -= 1;
        }

        assert_eq!(positions.len(), 3682);
    }

    #[test]
    fn part_02() {
        fn count_garden_plots(map: &Vec<Vec<char>>, start: (usize, usize), steps: usize) -> usize {
            let mut pos: HashSet<(usize, usize)> = HashSet::new();
            pos.insert(start);

            for _ in 0..steps {
                let mut new_pos: HashSet<(usize, usize)> = HashSet::new();
                for position in pos {
                    let (r, c) = position;
                    if r > 0 && map[r - 1][c] == '.' {
                        new_pos.insert((r - 1, c));
                    }
                    if r < map.len() - 1 && map[r + 1][c] == '.' {
                        new_pos.insert((r + 1, c));
                    }
                    if c > 0 && map[r][c - 1] == '.' {
                        new_pos.insert((r, c - 1));
                    }
                    if c < map[r].len() - 1 && map[r][c + 1] == '.' {
                        new_pos.insert((r, c + 1));
                    }
                }
                pos = new_pos;
            }
            pos.len()
        }

        let mut starting_point = (0, 0);
        let map = include_str!("input.txt")
            .lines()
            .enumerate()
            .map(|(r, l)| {
                l.chars()
                    .enumerate()
                    .map(|(c, char)| {
                        if char == 'S' {
                            starting_point = (r, c);
                            '.'
                        } else {
                            char
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        let map_size = map.len();
        let grid_size = 26501365 / map_size - 1;

        let even_maps_in_grid = ((grid_size + 1) / 2 * 2).pow(2);
        let odd_maps_in_grid = (grid_size / 2 * 2 + 1).pow(2);

        let total_in_base_grid = count_garden_plots(&map, starting_point, map_size * 2 + 1)
            * odd_maps_in_grid
            + count_garden_plots(&map, starting_point, map_size * 2) * even_maps_in_grid;

        let total_in_corners =
            count_garden_plots(&map, (map_size - 1, starting_point.1), map_size - 1)
                + count_garden_plots(&map, (starting_point.0, 0), map_size - 1)
                + count_garden_plots(&map, (0, starting_point.1), map_size - 1)
                + count_garden_plots(&map, (starting_point.0, map_size - 1), map_size - 1);

        let diagonal_total = ((grid_size + 1)
            * (count_garden_plots(&map, (map_size - 1, 0), map_size / 2 - 1)
                + count_garden_plots(&map, (0, 0), map_size / 2 - 1)
                + count_garden_plots(&map, (0, map_size - 1), map_size / 2 - 1)
                + count_garden_plots(&map, (map_size - 1, map_size - 1), map_size / 2 - 1)))
            + (grid_size
                * (count_garden_plots(&map, (map_size - 1, 0), map_size * 3 / 2 - 1)
                    + count_garden_plots(&map, (0, 0), map_size * 3 / 2 - 1)
                    + count_garden_plots(&map, (0, map_size - 1), map_size * 3 / 2 - 1)
                    + count_garden_plots(
                        &map,
                        (map_size - 1, map_size - 1),
                        map_size * 3 / 2 - 1,
                    )));

        assert_eq!(
            total_in_base_grid + total_in_corners + diagonal_total,
            609012263058042
        );
    }
}
