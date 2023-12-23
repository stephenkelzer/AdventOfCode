#[cfg(test)]
mod day_23 {
    use std::collections::VecDeque;

    use itertools::Itertools;

    fn get_bounded_neighbors(
        grid: &Vec<Vec<char>>,
        r: &usize,
        c: &usize,
        history: &Vec<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(|(dr, dc): &(isize, isize)| {
                let new_r = *r as isize + dr;
                let new_c = *c as isize + dc;
                if new_r < 0 || new_r >= grid.len() as isize {
                    return None;
                } else if new_c < 0 || new_c >= grid[0].len() as isize {
                    return None;
                }

                let new_r = new_r as usize;
                let new_c = new_c as usize;

                Some((new_r, new_c))
            })
            .filter_map(|(r, c)| {
                let spot_type = grid[r][c];
                if spot_type != '#' {
                    return Some((r, c));
                }
                None
            })
            .filter(|(r, c)| !history.contains(&(*r, *c)))
            .collect_vec()
    }

    #[test]
    fn part_01() {
        let grid = include_str!("input.txt")
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();
        let start = (0, 1);
        let finish = (grid.len() - 1, grid[0].len() - 2);
        // println!("finish: {finish:?}");

        let mut finishers: Vec<Vec<(usize, usize)>> = vec![];

        let mut queue: VecDeque<((usize, usize), Vec<(usize, usize)>)> =
            VecDeque::from([(start, vec![])]);

        while let Some(((mut r, mut c), mut history)) = queue.pop_front() {
            // println!("history: {history:?}");
            if r == finish.0 && c == finish.1 {
                finishers.push(history.clone());
                continue;
                // panic!("we've made it");
            }

            // println!(
            //     "type: {:?} | ({:?}, {:?}) | h_l: {:?}",
            //     grid[r][c],
            //     r,
            //     c,
            //     history.len()
            // );

            history.push((r, c));
            match grid[r][c] {
                '<' => {
                    // history.push((r, c));
                    c -= 1;
                    if !history.contains(&(r, c)) {
                        queue.push_back(((r, c), history.clone()));
                    }
                }
                '>' => {
                    // history.push((r, c));
                    c += 1;
                    if !history.contains(&(r, c)) {
                        queue.push_back(((r, c), history.clone()));
                    }
                }
                '^' => {
                    // history.push((r, c));
                    r -= 1;
                    if !history.contains(&(r, c)) {
                        queue.push_back(((r, c), history.clone()));
                    }
                }
                'v' => {
                    // history.push((r, c));
                    r += 1;
                    if !history.contains(&(r, c)) {
                        queue.push_back(((r, c), history.clone()));
                    }
                }
                '.' => {
                    // history.push((r, c));

                    get_bounded_neighbors(&grid, &r, &c, &history)
                        .iter()
                        .for_each(|(nr, nc)| {
                            // println!("neighbor: {:?}", (nr, nc));
                            queue.push_back(((*nr, *nc), history.clone()));
                        });
                }
                _ => unreachable!(),
            }

            // get_bounded_neighbors(&grid, &r, &c, &history)
            //     .iter()
            //     .for_each(|(nr, nc)| {
            //         println!("neighbor: {:?}", (nr, nc));
            //         queue.push_back(((*nr, *nc), history.clone()));
            //     });

            // if history.len() == 20 {
            //     break;
            // }
        }

        // for h in &finishers {
        //     for (r, row) in grid.iter().enumerate() {
        //         println!(
        //             "{:?}",
        //             row.iter()
        //                 .enumerate()
        //                 .map(|(c, _)| {
        //                     if h.contains(&(r, c)) {
        //                         return '0';
        //                     } else {
        //                         return grid[r][c];
        //                     }
        //                 })
        //                 .join("")
        //         );
        //         // for (c, col) in row.iter().enumerate() {
        //         //     if h.contains(&(r, c)) {
        //         //         print!("0");
        //         //     } else {
        //         //         print!("{:?}", grid[r][c]);
        //         //     }
        //         // }
        //         // println!()
        //     }
        //     println!("finisher: {:?}", h.len());
        // }

        let answer = finishers.iter().map(|x| x.len()).max().unwrap();

        assert_eq!(answer, 2086);
    }

    #[test]
    #[ignore = "needs optimization (runs in roughly 2 minutes at the moment)"]
    fn part_02() {
        let grid = include_str!("input.txt")
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();

        fn dfs(
            grid: &Vec<Vec<char>>,
            seen: &mut Vec<Vec<bool>>,
            (r, c): (usize, usize),
            dist: usize,
            max_dist: &mut usize,
        ) {
            if r == grid.len() - 1 {
                *max_dist = (*max_dist).max(dist);
            }

            for &(dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)].as_slice() {
                let rr = (r as isize + dr) as usize;
                let cc = (c as isize + dc) as usize;
                let Some(&tile) = grid.get(rr).and_then(|row| row.get(cc)) else {
                    continue;
                };
                if tile == '#' || seen[rr][cc] {
                    continue;
                }
                seen[rr][cc] = true;
                dfs(grid, seen, (rr, cc), dist + 1, max_dist);
                seen[rr][cc] = false;
            }
        }

        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];

        let mut answer = 0;
        dfs(&grid, &mut seen, (0, 1), 0, &mut answer);
        assert_eq!(answer, 6526);
    }
}
