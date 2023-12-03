#[cfg(test)]
mod day_3 {
    pub struct Grid {
        pub grid: Vec<Vec<char>>,
    }

    impl Default for Grid {
        fn default() -> Grid {
            Grid {
                grid: include_str!("puzzle.txt")
                    .split("\n")
                    .map(|line| line.trim().chars().collect())
                    .collect(),
            }
        }
    }

    impl Grid {
        pub fn get_adjacent(&self, row: usize, col: usize) -> Vec<(char, usize, usize)> {
            let mut adjacent = Vec::new();

            // top left
            if row > 0 && col > 0 {
                adjacent.push((self.grid[row - 1][col - 1], row - 1, col - 1));
            }

            // top
            if row > 0 {
                adjacent.push((self.grid[row - 1][col], row - 1, col));
            }

            // top right
            if row > 0 && col < self.grid[row].len() - 1 {
                adjacent.push((self.grid[row - 1][col + 1], row - 1, col + 1));
            }

            // left
            if col > 0 {
                adjacent.push((self.grid[row][col - 1], row, col - 1));
            }

            // right
            if col < self.grid[row].len() - 1 {
                adjacent.push((self.grid[row][col + 1], row, col + 1));
            }

            // bottom left
            if row < self.grid.len() - 1 && col > 0 {
                adjacent.push((self.grid[row + 1][col - 1], row + 1, col - 1));
            }

            // bottom
            if row < self.grid.len() - 1 {
                adjacent.push((self.grid[row + 1][col], row + 1, col));
            }

            // bottom right
            if row < self.grid.len() - 1 && col < self.grid[row].len() - 1 {
                adjacent.push((self.grid[row + 1][col + 1], row + 1, col + 1));
            }

            adjacent
        }

        pub fn is_eligible(&self, row: usize, col: usize) -> bool {
            let adjacent = self.get_adjacent(row, col);
            adjacent
                .iter()
                .map(|x| x.0)
                .any(|x| !x.eq(&'.') && !x.is_numeric())
        }

        pub fn get_part_number_at(&self, row: usize, col: usize) -> Option<(usize, usize, usize)> {
            let at_char = self.grid[row][col];

            if !at_char.is_numeric() {
                return None;
            }

            let mut part_number_starting_c = col;
            loop {
                if self.grid[row][(part_number_starting_c) - 1].is_numeric() {
                    part_number_starting_c -= 1;
                } else {
                    break;
                }

                if part_number_starting_c == 0 {
                    break;
                }
            }

            let mut curr_c: usize = part_number_starting_c as usize;
            let mut curr_part_number = String::new();
            while curr_c < self.grid[row].len() && self.grid[row][curr_c].is_numeric() {
                curr_part_number.push(self.grid[row][curr_c]);
                curr_c += 1;
            }

            Some((
                curr_part_number
                    .parse::<usize>()
                    .expect("Failed to parse string into usize"),
                row,
                part_number_starting_c,
            ))
        }
    }

    #[test]
    fn part_one() {
        let grid = Grid::default();

        let mut countable_numbers = Vec::new();
        let mut curr_part_number = String::new();
        let mut curr_part_number_eligible = false;

        for (r, row) in grid.grid.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if col.is_numeric() {
                    curr_part_number.push(*col);

                    if grid.is_eligible(r, c) {
                        curr_part_number_eligible = true;
                    }
                } else {
                    if curr_part_number.len() > 0 {
                        let num = curr_part_number
                            .parse::<usize>()
                            .expect("Failed to parse string into usize");

                        if curr_part_number_eligible {
                            countable_numbers.push(num);
                        }

                        curr_part_number.clear();
                        curr_part_number_eligible = false;
                    }
                }
            }
        }

        assert_eq!(countable_numbers.iter().sum::<usize>(), 527364);
    }

    #[test]
    fn part_two() {
        let grid = Grid::default();

        let mut gear_ratios = Vec::new();

        for (r, row) in grid.grid.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if col.eq(&'*') {
                    let neighbors = grid.get_adjacent(r, c);
                    let mut part_numbers = Vec::<(usize, usize, usize)>::new(); // (part_number, row, col)

                    neighbors
                        .iter()
                        .for_each(|x| match grid.get_part_number_at(x.1, x.2) {
                            Some((part_number, part_number_starting_r, part_number_starting_c)) => {
                                if !part_numbers.contains(&(
                                    part_number,
                                    part_number_starting_r,
                                    part_number_starting_c,
                                )) {
                                    part_numbers.push((
                                        part_number,
                                        part_number_starting_r,
                                        part_number_starting_c,
                                    ))
                                }
                            }
                            None => {}
                        });

                    if part_numbers.len() == 2 {
                        gear_ratios.push(part_numbers.iter().map(|x| x.0).product());
                    }
                }
            }
        }

        assert_eq!(gear_ratios.iter().sum::<usize>(), 79026871);
    }
}
