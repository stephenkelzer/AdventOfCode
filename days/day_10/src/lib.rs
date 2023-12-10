#[cfg(test)]
mod day_10 {
    use std::collections::HashMap;

    use itertools::Itertools;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Point(usize, usize);

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

        let get_neighbors = |Point(r, c)| -> Vec<Point> {
            let char = grid[r][c];
            if &char == &'S' {
                return NEIGHBORS_MAP
                    .iter()
                    .filter_map(|[r_mod, c_mod]| {
                        let modded_r = r as isize + r_mod;
                        let modded_c = c as isize + c_mod;
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
                        let [[dir_1_r, dir_1_c], [dir_2_r, dir_2_c]] =
                            char_directions.get(&char).unwrap();

                        let modded_r = (*spot_r as isize + dir_1_r) as usize;
                        let modded_c = (*spot_c as isize + dir_1_c) as usize;
                        if modded_r == r && modded_c == c {
                            return true;
                        }

                        let modded_r = (*spot_r as isize + dir_2_r) as usize;
                        let modded_c = (*spot_c as isize + dir_2_c) as usize;
                        if modded_r == r && modded_c == c {
                            return true;
                        }

                        false
                    })
                    .map(|x| x.0)
                    .collect_vec();
            }

            let mut neighbors = vec![];
            let [[dir_1_r, dir_1_c], [dir_2_r, dir_2_c]] = char_directions.get(&char).unwrap();

            // println!("{:?}|{:?}|{:?}|{:?}", dir_1_r, dir_1_c, dir_2_r, dir_2_c);
            let modded_r = (r as isize + dir_1_r) as usize;
            let modded_c = (c as isize + dir_1_c) as usize;
            neighbors.push(Point(modded_r, modded_c));

            let modded_r = (r as isize + dir_2_r) as usize;
            let modded_c = (c as isize + dir_2_c) as usize;
            neighbors.push(Point(modded_r, modded_c));

            neighbors
        };

        let starting_spots = get_neighbors(position);

        let mut count = 1;
        let mut spot_1 = starting_spots[0];
        let mut spot_2 = starting_spots[1];
        let mut spot_1_history = vec![position, starting_spots[0]];
        let mut spot_2_history = vec![position, starting_spots[1]];

        while spot_1 != spot_2 {
            let spot_1_neighbors = get_neighbors(spot_1);
            let next_spot_1 = spot_1_neighbors
                .iter()
                .find(|x| !spot_1_history.contains(x))
                .unwrap();

            let spot_2_neighbors = get_neighbors(spot_2);
            let next_spot_2 = spot_2_neighbors
                .iter()
                .find(|x| !spot_2_history.contains(x))
                .unwrap();

            spot_1 = *next_spot_1;
            spot_1_history.push(spot_1);
            spot_2 = *next_spot_2;
            spot_2_history.push(spot_2);

            count += 1;
        }

        assert_eq!(count, 6846);
    }

    #[test]
    fn part_two() {
        let lines = include_str!("input.txt").lines();

        assert_eq!(123, 123);
    }
}
