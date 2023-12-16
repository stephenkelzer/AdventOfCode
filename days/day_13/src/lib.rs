#[cfg(test)]
mod day_13 {
    use itertools::Itertools;

    #[test]
    fn part_01() {
        let grids = include_str!("input.txt")
            .split_terminator("\n\n")
            .map(|chunk| chunk.split('\n').collect_vec())
            .collect_vec();

        /// HORIZONTAL REFLECTION LINES
        fn check_for_row_reflection(pattern: &Vec<&str>) -> Option<usize> {
            let validate = |idx: usize| -> bool {
                let mut l_idx: isize = idx as isize;
                let mut r_idx = idx + 1;

                loop {
                    // println!("checking: {:?}|{:?}", l_idx, r_idx);
                    let l = pattern.iter().nth(l_idx as usize);
                    let r = pattern.iter().nth(r_idx);
                    if l.is_none() || r.is_none() {
                        // made it to an edge
                        return true;
                    }

                    if l != r {
                        return false;
                    }

                    l_idx -= 1;
                    r_idx += 1;
                }
            };

            pattern
                .windows(2)
                .enumerate()
                // .inspect(|x| println!("{x:?}"))
                .find_map(
                    |(index, rows_window)| match rows_window[0] == rows_window[1] {
                        true => match validate(index) {
                            true => Some(index),
                            false => None,
                        },
                        false => None,
                    },
                )
        }

        /// VERTICAL REFLECTION LINES
        fn check_for_column_reflection(pattern: &Vec<&str>) -> Option<usize> {
            let pivot_column = |column_index: usize| -> Option<String> {
                let column_text = pattern
                    .iter()
                    .filter_map(|r| r.chars().nth(column_index))
                    .join("");

                match column_text.is_empty() {
                    true => None,
                    false => Some(column_text),
                }
            };

            let validate = |idx: usize| -> bool {
                let mut l_idx: isize = idx as isize;
                let mut r_idx = idx + 1;

                loop {
                    let l = pivot_column(l_idx as usize);
                    let r = pivot_column(r_idx);

                    // println!("checking: {:?}|{:?}", l_idx, r_idx);
                    if l.is_none() || r.is_none() {
                        // made it to an edge
                        return true;
                    }

                    if l != r {
                        return false;
                    }

                    l_idx -= 1;
                    r_idx += 1;
                }
            };

            let column_char_count = pattern[0].len();
            let test = (0..column_char_count)
                .collect_vec()
                .windows(2)
                .enumerate()
                .find_map(|(index, columns_window)| {
                    let l_column = &pivot_column(columns_window[0]).expect("CRAP!");
                    let r_column = &pivot_column(columns_window[1]).expect("CRAP!");

                    match l_column == r_column {
                        true => match validate(index) {
                            true => Some(index),
                            false => None,
                        },
                        false => None,
                    }
                });
            // println!("TEST: {test:?}");

            test
        }
        // println!("{:?}", reflections);

        let answer = grids
            .iter()
            .map(|pattern| match check_for_row_reflection(pattern) {
                Some(horizontal_reflection_index) => (horizontal_reflection_index + 1) * 100,
                None => match check_for_column_reflection(pattern) {
                    Some(vertical_reflection_index) => vertical_reflection_index + 1,
                    None => 0,
                },
            })
            .sum::<usize>();

        assert_eq!(answer, 34918);
    }

    #[test]
    fn part_02() {
        let grids = include_str!("input.txt")
            .split_terminator("\n\n")
            .map(|chunk| chunk.lines().map(|l| l.chars().collect_vec()).collect_vec())
            .collect_vec();

        fn compare_rows(grid: &Vec<Vec<char>>, row1: usize, row2: usize) -> isize {
            let row1 = grid[row1].iter().collect::<String>();
            let row1 = row1.chars();

            let row2 = grid[row2].iter().collect::<String>();
            let row2 = row2.chars();

            row1.zip(row2).filter(|&(c1, c2)| c1 != c2).count() as isize
        }

        fn compare_cols(grid: &Vec<Vec<char>>, col1: usize, col2: usize) -> isize {
            let col1 = grid.iter().map(|row| row[col1]).collect::<String>();
            let col1 = col1.chars();

            let col2 = grid.iter().map(|row| row[col2]).collect::<String>();
            let col2 = col2.chars();

            col1.zip(col2).filter(|&(c1, c2)| c1 != c2).count() as isize
        }

        /// (horizontal_reflection_index, vertical_reflection_index)
        fn find_reflections(grid: Vec<Vec<char>>) -> (usize, usize) {
            for col in 1..grid[0].len() {
                let mut l = col as isize - 1;
                let mut r = col;
                let mut diffs = 0;

                while l >= 0 && r < grid[0].len() {
                    if diffs > 1 {
                        break;
                    }

                    diffs += compare_cols(&grid, l as usize, r);
                    l -= 1;
                    r += 1;
                }

                if diffs == 1 {
                    return (0, col);
                }
            }

            for row in 1..grid.len() {
                let mut u = row as isize - 1;
                let mut d = row;
                let mut diffs = 0;

                while u >= 0 && d < grid.len() {
                    if diffs > 1 {
                        break;
                    }
                    diffs += compare_rows(&grid, u as usize, d);
                    u -= 1;
                    d += 1;
                }

                if diffs == 1 {
                    return (row, 0);
                }
            }

            (0, 0)
        }

        let answer = grids
            .iter()
            .map(|g| {
                let (h_ref_idx, v_ref_idx) = find_reflections(g.to_vec());
                (h_ref_idx * 100) + v_ref_idx
            })
            .sum::<usize>();

        assert_eq!(answer, 33054);
    }
}
