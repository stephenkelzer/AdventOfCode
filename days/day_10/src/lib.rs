#[cfg(test)]
mod day_10 {
    use itertools::Itertools;
    use std::collections::{HashMap, VecDeque};

    const NEIGHBORS_MAP: [[isize; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Point(usize, usize);

    #[test]
    fn part_one() {
        let lines = include_str!("input.txt").lines();

        let char_directions: HashMap<char, [[isize; 2]; 2]> = HashMap::from([
            ('F', [[0, 1], [1, 0]]),
            ('7', [[0, -1], [1, 0]]),
            ('L', [[-1, 0], [0, 1]]),
            ('J', [[-1, 0], [0, -1]]),
            ('-', [[0, -1], [0, 1]]),
            ('|', [[-1, 0], [1, 0]]),
        ]);

        let mut position: Point = Point(0, 0);
        let grid = lines
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, x)| {
                        if x == 'S' {
                            position = Point(i, j)
                        }

                        x
                    })
                    .collect_vec()
            })
            .collect_vec();
        let grid_w = grid[0].len();
        let grid_h = grid.len();

        let is_inbounds = |Point(r, c)| r <= grid_h && c <= grid_w;

        let starting_spots = NEIGHBORS_MAP
            .iter()
            .filter_map(|[r_mod, c_mod]| {
                let modded_r = position.0 as isize + r_mod;
                let modded_c = position.1 as isize + c_mod;
                if modded_r.is_negative() || modded_c.is_negative() {
                    return None;
                }
                let modded_r = modded_r as usize;
                let modded_c = modded_c as usize;

                let x = Point(modded_r, modded_c);

                if is_inbounds(x.clone()) && grid[modded_r][modded_c] != '.' {
                    let char = grid[modded_r][modded_c];
                    if char != '.' {
                        return Some((x, char));
                    }
                }

                None
            })
            .filter(|(Point(spot_r, spot_c), char)| {
                let [[dir_1_r, dir_1_c], [dir_2_r, dir_2_c]] = char_directions.get(&char).unwrap();

                let modded_r = (*spot_r as isize + dir_1_r) as usize;
                let modded_c = (*spot_c as isize + dir_1_c) as usize;
                if modded_r == position.0 && modded_c == position.1 {
                    return true;
                }

                let modded_r = (*spot_r as isize + dir_2_r) as usize;
                let modded_c = (*spot_c as isize + dir_2_c) as usize;
                if modded_r == position.0 && modded_c == position.1 {
                    return true;
                }

                false
            })
            .map(|x| x.0)
            .collect_vec();

        let get_neighbors = |Point(r, c)| -> Vec<Point> {
            let char = grid[r][c];
            let mut neighbors = vec![];
            let [[dir_1_r, dir_1_c], [dir_2_r, dir_2_c]] = char_directions.get(&char).unwrap();

            let modded_r = (r as isize + dir_1_r) as usize;
            let modded_c = (c as isize + dir_1_c) as usize;
            neighbors.push(Point(modded_r, modded_c));

            let modded_r = (r as isize + dir_2_r) as usize;
            let modded_c = (c as isize + dir_2_c) as usize;
            neighbors.push(Point(modded_r, modded_c));

            neighbors
        };

        let mut spot_1_history = vec![position, starting_spots[0]];
        let mut spot_2_history = vec![position, starting_spots[1]];

        while spot_1_history.last().unwrap() != spot_2_history.last().unwrap() {
            spot_1_history.push(
                *get_neighbors(*spot_1_history.last().unwrap())
                    .iter()
                    .find(|x| !spot_1_history.contains(x))
                    .unwrap(),
            );

            spot_2_history.push(
                *get_neighbors(*spot_2_history.last().unwrap())
                    .iter()
                    .find(|x| !spot_2_history.contains(x))
                    .unwrap(),
            );
        }

        assert_eq!(spot_1_history.len() - 1, 6846);
    }

    #[test]
    fn part_two() {
        let lines = include_str!("input.txt").lines();

        let char_directions: HashMap<char, [[isize; 2]; 2]> = HashMap::from([
            ('F', [[0, 1], [1, 0]]),
            ('7', [[0, -1], [1, 0]]),
            ('L', [[-1, 0], [0, 1]]),
            ('J', [[-1, 0], [0, -1]]),
            ('-', [[0, -1], [0, 1]]),
            ('|', [[-1, 0], [1, 0]]),
        ]);

        let mut position: Point = Point(0, 0);
        let grid = lines
            .enumerate()
            .map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .map(|(j, x)| {
                        if x == 'S' {
                            position = Point(i, j)
                        }

                        x
                    })
                    .collect_vec()
            })
            .collect_vec();
        let grid_w = grid[0].len();
        let grid_h = grid.len();

        let is_inbounds = |Point(r, c)| r <= grid_h && c <= grid_w;

        let starting_spots = NEIGHBORS_MAP
            .iter()
            .filter_map(|[r_mod, c_mod]| {
                let modded_r = position.0 as isize + r_mod;
                let modded_c = position.1 as isize + c_mod;
                if modded_r.is_negative() || modded_c.is_negative() {
                    return None;
                }
                let modded_r = modded_r as usize;
                let modded_c = modded_c as usize;

                let x = Point(modded_r, modded_c);

                if is_inbounds(x.clone()) && grid[modded_r][modded_c] != '.' {
                    let char = grid[modded_r][modded_c];
                    if char != '.' {
                        return Some((x, char));
                    }
                }

                None
            })
            .filter(|(Point(spot_r, spot_c), char)| {
                let [[dir_1_r, dir_1_c], [dir_2_r, dir_2_c]] = char_directions.get(&char).unwrap();

                let modded_r = (*spot_r as isize + dir_1_r) as usize;
                let modded_c = (*spot_c as isize + dir_1_c) as usize;
                if modded_r == position.0 && modded_c == position.1 {
                    return true;
                }

                let modded_r = (*spot_r as isize + dir_2_r) as usize;
                let modded_c = (*spot_c as isize + dir_2_c) as usize;
                if modded_r == position.0 && modded_c == position.1 {
                    return true;
                }

                false
            })
            .map(|x| x.0)
            .collect_vec();

        let get_neighbors = |Point(r, c)| -> Vec<Point> {
            let char = grid[r][c];
            let mut neighbors = vec![];
            let [[dir_1_r, dir_1_c], [dir_2_r, dir_2_c]] = char_directions.get(&char).unwrap();

            let modded_r = (r as isize + dir_1_r) as usize;
            let modded_c = (c as isize + dir_1_c) as usize;
            neighbors.push(Point(modded_r, modded_c));

            let modded_r = (r as isize + dir_2_r) as usize;
            let modded_c = (c as isize + dir_2_c) as usize;
            neighbors.push(Point(modded_r, modded_c));

            neighbors
        };

        let mut vertices: VecDeque<Point> =
            VecDeque::from(vec![starting_spots[0], position, starting_spots[1]]);

        let mut found_end = false;

        while !found_end {
            if let Some(point) = get_neighbors(*vertices.front().unwrap())
                .iter()
                .find(|x| !vertices.contains(x))
            {
                vertices.push_front(*point);
            } else {
                found_end = true;
            }

            if let Some(point) = get_neighbors(*vertices.back().unwrap())
                .iter()
                .find(|x| !vertices.contains(x))
            {
                vertices.push_back(*point);
            } else {
                found_end = true;
            }
        }

        let loop_length = vertices.len();
        let mut area = 0.0;

        for i in 0..loop_length {
            let j = (i + 1) % loop_length;
            area += (vertices[i].0 * vertices[j].1) as f64 - (vertices[j].0 * vertices[i].1) as f64;
        }

        let answer = (area.abs() / 2.0) + 1. - loop_length as f64 / 2.;

        assert_eq!(answer, 325.)
    }
}
