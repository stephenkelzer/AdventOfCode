#[cfg(test)]
mod day_16 {
    use itertools::Itertools;
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Pos(isize, isize, Dir);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Dir {
        N,
        E,
        S,
        W,
    }

    #[test]
    fn part_01() {
        let mut cache = HashMap::new();
        let mut energized_tiles: HashSet<(isize, isize)> = HashSet::new();

        let grid = include_str!("input.txt")
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();

        let height: isize = grid.len() as isize;
        let width: isize = grid[0].len() as isize;

        let is_valid_spot =
            |r: &isize, c: &isize| -> bool { r >= &0 && c >= &0 && r < &height && c < &width };

        let get_spot = |r: isize, c: isize| -> Option<char> {
            if is_valid_spot(&r, &c) {
                return Some(grid[r as usize][c as usize]);
            }

            None
        };

        let mut beams_queue = vec![Pos(0, 0, Dir::E)];

        while beams_queue.len() > 0 {
            let beams_to_process = beams_queue
                .iter()
                .cloned()
                .filter(|x| {
                    if cache.contains_key(x) {
                        return false;
                    }
                    true
                })
                .collect_vec();
            beams_queue.clear();

            let next = beams_to_process
                .iter()
                .flat_map(|__beam| {
                    let Pos(r, c, heading) = *__beam;

                    if let Some(ch) = get_spot(r, c) {
                        energized_tiles.insert((r, c));

                        let potentials: Vec<Dir> = match (ch, heading) {
                            ('.', _) => vec![heading],
                            //
                            ('|', Dir::W) | ('|', Dir::E) => vec![Dir::N, Dir::S],
                            ('|', _) => vec![heading],
                            //
                            ('-', Dir::N) | ('-', Dir::S) => vec![Dir::E, Dir::W],
                            ('-', _) => vec![heading],
                            //
                            ('\\', Dir::N) => vec![Dir::W],
                            ('\\', Dir::E) => vec![Dir::S],
                            ('\\', Dir::S) => vec![Dir::E],
                            ('\\', Dir::W) => vec![Dir::N],
                            //
                            ('/', Dir::N) => vec![Dir::E],
                            ('/', Dir::E) => vec![Dir::N],
                            ('/', Dir::S) => vec![Dir::W],
                            ('/', Dir::W) => vec![Dir::S],
                            _ => unreachable!("Can't handle: {}", c),
                        };

                        potentials
                            .into_iter()
                            .map(|dir| {
                                let next = match dir {
                                    Dir::N => Pos(r - 1, c, dir),
                                    Dir::E => Pos(r, c + 1, dir),
                                    Dir::S => Pos(r + 1, c, dir),
                                    Dir::W => Pos(r, c - 1, dir),
                                };
                                cache.insert(__beam.clone(), next);
                                next
                            })
                            .collect_vec()
                    } else {
                        vec![]
                    }
                })
                .collect_vec();
            next.iter().for_each(|x| beams_queue.push(*x));
        }

        assert_eq!(energized_tiles.len(), 7477);
    }

    #[test]
    fn part_02() {
        fn go(
            board: &Vec<Vec<char>>,
            dir_enum: &Dir,
            idx: (usize, usize),
            visited: &mut HashMap<(usize, usize), Vec<Dir>>,
        ) {
            let dir = get_dir(&dir_enum);
            let mut curr_r = idx.0 as i32;
            let mut curr_c = idx.1 as i32;
            let mut current_char = board[idx.0][idx.1];
            let row_len = board.len() as i32;
            let col_len = board[0].len() as i32;
            let specials = ['/', '\\', '|', '-'];
            while !specials.contains(&current_char) {
                // board[curr_r as usize][curr_c as usize] = '#';
                visited.insert((curr_r as usize, curr_c as usize), vec![]);
                curr_r += dir.0;
                curr_c += dir.1;
                if curr_r < 0 || curr_c < 0 || curr_r >= row_len || curr_c >= col_len {
                    return;
                }
                current_char = board[curr_r as usize][curr_c as usize];
            }

            let vec = visited
                .entry((curr_r as usize, curr_c as usize))
                .or_insert(vec![]);
            if current_char == '/' {
                let dir_enum = match dir_enum {
                    Dir::E => Dir::N,
                    Dir::N => Dir::E,
                    Dir::W => Dir::S,
                    Dir::S => Dir::W,
                };
                if vec.contains(&dir_enum) {
                    return;
                }
                vec.push(dir_enum.clone());
                let dir = get_dir(&dir_enum);
                let new_r = curr_r + dir.0;
                let new_c = curr_c + dir.1;
                if in_range(new_r, new_c, row_len, col_len) {
                    go(board, &dir_enum, (new_r as usize, new_c as usize), visited);
                }
            } else if current_char == '\\' {
                let dir_enum = match dir_enum {
                    Dir::E => Dir::S,
                    Dir::S => Dir::E,
                    Dir::W => Dir::N,
                    Dir::N => Dir::W,
                };
                if vec.contains(&dir_enum) {
                    return;
                }
                vec.push(dir_enum.clone());
                let dir = get_dir(&dir_enum);
                let new_r = curr_r + dir.0;
                let new_c = curr_c + dir.1;
                if in_range(new_r, new_c, row_len, col_len) {
                    go(board, &dir_enum, (new_r as usize, new_c as usize), visited);
                }
            } else if current_char == '|' {
                let N = get_dir(&Dir::N);
                let S = get_dir(&Dir::S);
                let N_row = curr_r + N.0;
                let N_col = curr_c + N.1;
                if in_range(N_row, N_col, row_len, col_len) && !matches!(dir_enum, Dir::S) {
                    go(board, &Dir::N, (N_row as usize, N_col as usize), visited);
                }
                let S_row = curr_r + S.0;
                let S_col = curr_c + S.1;
                if in_range(S_row, S_col, row_len, col_len) && !matches!(dir_enum, Dir::N) {
                    go(board, &Dir::S, (S_row as usize, S_col as usize), visited);
                }
            } else if current_char == '-' {
                let W = get_dir(&Dir::W);
                let E = get_dir(&Dir::E);
                let W_row = curr_r + W.0;
                let W_col = curr_c + W.1;
                if in_range(W_row, W_col, row_len, col_len) && !matches!(dir_enum, Dir::E) {
                    go(board, &Dir::W, (W_row as usize, W_col as usize), visited);
                }
                let E_row = curr_r + E.0;
                let E_col = curr_c + E.1;
                if in_range(E_row, E_col, row_len, col_len) && !matches!(dir_enum, Dir::W) {
                    go(board, &Dir::E, (E_row as usize, E_col as usize), visited);
                }
            }
        }

        fn in_range(row: i32, col: i32, row_len: i32, col_len: i32) -> bool {
            return 0 <= row && row < row_len && 0 <= col && col < col_len;
        }

        fn get_dir(dir: &Dir) -> (i32, i32) {
            match dir {
                Dir::N => (-1, 0),
                Dir::S => (1, 0),
                Dir::W => (0, -1),
                Dir::E => (0, 1),
            }
        }

        let mut board: Vec<Vec<char>> = Vec::new();
        for line in include_str!("input.txt").lines() {
            board.push(line.chars().collect());
        }
        let row_len = board.len();
        let col_len = board[0].len();
        let mut max = 0;
        // S
        for j in 0..col_len {
            let mut visited = HashMap::new();
            go(&board, &Dir::S, (0, j), &mut visited);
            max = max.max(visited.len());
        }
        // N
        for j in 0..col_len {
            let mut visited = HashMap::new();
            go(&board, &Dir::N, (row_len - 1, j), &mut visited);
            max = max.max(visited.len());
        }
        // W
        for i in 0..row_len {
            let mut visited = HashMap::new();
            go(&board, &Dir::W, (i, col_len - 1), &mut visited);
            max = max.max(visited.len());
        }
        // E
        for i in 0..row_len {
            let mut visited = HashMap::new();
            go(&board, &Dir::E, (i, 0), &mut visited);
            max = max.max(visited.len());
        }
        // println!("{max}");
        // max.to_string()

        assert_eq!(max, 7853);
    }
}
