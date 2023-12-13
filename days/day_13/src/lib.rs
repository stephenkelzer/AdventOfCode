#[cfg(test)]
mod day_13 {
    use itertools::Itertools;

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
        println!("TEST: {test:?}");

        test
    }

    #[test]
    fn part_01() {
        let grids = include_str!("input.txt")
            .split_terminator("\n\n")
            .map(|chunk| chunk.split("\n").collect_vec())
            .collect_vec();
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
    fn fun() {
        let str = &"test";
        assert_eq!(str.chars().nth(2).unwrap(), 's');
    }

    #[test]
    fn part_02() {
        // let lines = include_str!("input.txt").lines();

        assert_eq!(123, 123);
    }
}
