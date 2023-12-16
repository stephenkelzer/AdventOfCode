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

        while !beams_queue.is_empty() {
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
                                cache.insert(*__beam, next);
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
            let dir = get_dir(dir_enum);
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
                .or_default();
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
                vec.push(dir_enum);
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
                vec.push(dir_enum);
                let dir = get_dir(&dir_enum);
                let new_r = curr_r + dir.0;
                let new_c = curr_c + dir.1;
                if in_range(new_r, new_c, row_len, col_len) {
                    go(board, &dir_enum, (new_r as usize, new_c as usize), visited);
                }
            } else if current_char == '|' {
                let n = get_dir(&Dir::N);
                let s = get_dir(&Dir::S);
                let n_row = curr_r + n.0;
                let n_col = curr_c + n.1;
                if in_range(n_row, n_col, row_len, col_len) && !matches!(dir_enum, Dir::S) {
                    go(board, &Dir::N, (n_row as usize, n_col as usize), visited);
                }
                let s_row = curr_r + s.0;
                let s_col = curr_c + s.1;
                if in_range(s_row, s_col, row_len, col_len) && !matches!(dir_enum, Dir::N) {
                    go(board, &Dir::S, (s_row as usize, s_col as usize), visited);
                }
            } else if current_char == '-' {
                let w = get_dir(&Dir::W);
                let e = get_dir(&Dir::E);
                let w_row = curr_r + w.0;
                let w_col = curr_c + w.1;
                if in_range(w_row, w_col, row_len, col_len) && !matches!(dir_enum, Dir::E) {
                    go(board, &Dir::W, (w_row as usize, w_col as usize), visited);
                }
                let e_row = curr_r + e.0;
                let e_col = curr_c + e.1;
                if in_range(e_row, e_col, row_len, col_len) && !matches!(dir_enum, Dir::W) {
                    go(board, &Dir::E, (e_row as usize, e_col as usize), visited);
                }
            }
        }

        fn in_range(row: i32, col: i32, row_len: i32, col_len: i32) -> bool {
            0 <= row && row < row_len && 0 <= col && col < col_len
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
