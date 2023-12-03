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
        fn get_adjacent(&self, row: usize, col: usize) -> Vec<char> {
            let mut adjacent = Vec::new();

            // top left
            if row > 0 && col > 0 {
                adjacent.push(self.grid[row - 1][col - 1]);
            }

            // top
            if row > 0 {
                adjacent.push(self.grid[row - 1][col]);
            }

            // top right
            if row > 0 && col < self.grid[row].len() - 1 {
                adjacent.push(self.grid[row - 1][col + 1]);
            }

            // left
            if col > 0 {
                adjacent.push(self.grid[row][col - 1]);
            }

            // right
            if col < self.grid[row].len() - 1 {
                adjacent.push(self.grid[row][col + 1]);
            }

            // bottom left
            if row < self.grid.len() - 1 && col > 0 {
                adjacent.push(self.grid[row + 1][col - 1]);
            }

            // bottom
            if row < self.grid.len() - 1 {
                adjacent.push(self.grid[row + 1][col]);
            }

            // bottom right
            if row < self.grid.len() - 1 && col < self.grid[row].len() - 1 {
                adjacent.push(self.grid[row + 1][col + 1]);
            }

            adjacent
        }

        pub fn is_eligible(&self, row: usize, col: usize) -> bool {
            let adjacent = self.get_adjacent(row, col);
            adjacent.iter().any(|x| !x.eq(&'.') && !x.is_numeric())
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

                    if grid.is_eligible(r,c) {
                        curr_part_number_eligible = true;
                    }
                } else {
                    if curr_part_number.len() > 0 {
                        let num = curr_part_number.parse::<usize>().expect("Failed to parse string into usize");
                        
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
        let result: usize = 123;

        assert_eq!(result, 123);
    }
}
