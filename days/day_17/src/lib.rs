#[cfg(test)]
mod day_17 {
    use itertools::Itertools;
    use std::collections::{BinaryHeap, HashMap};

    /// Using Dijkstra's algorithm for optimal path detection
    /// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
    fn dijkstra(grid: Vec<Vec<usize>>, min_steps: isize, max_steps: isize) -> isize {
        let grid_height = grid.len();
        let grid_width = grid[0].len();
        let start = (0, 0);
        let finish = (grid_height - 1, grid_width - 1);

        let mut dist = HashMap::from([((start.0, start.1, (0, 0)), 0)]);
        let mut prev = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);

        while let Some((cost, (r, c, d))) = prev.pop() {
            if (r, c) == finish {
                // reached the end
                return -cost;
            }

            if dist.get(&(r, c, d)).is_some_and(|&c| -cost > c) {
                continue; // cache hit
            }

            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if d == (dr, dc) || d == (-dr, -dc) {
                    continue; // can't turn around
                }

                let mut next_cost = -cost;

                // ensure we only go up to the maximum distance each hop with max_steps
                for distance in 1..=max_steps {
                    let rr = (r as isize + dr * distance) as usize;
                    let cc = (c as isize + dc * distance) as usize;

                    if rr >= grid_height || cc >= grid_width {
                        // out of bounds
                        continue;
                    }

                    next_cost += (grid[rr][cc] - 0) as isize;

                    let key = (rr, cc, (dr, dc));

                    if distance < min_steps {
                        // must respect our minimum steps requirement
                        continue;
                    }

                    if next_cost < *dist.get(&key).unwrap_or(&isize::MAX) {
                        dist.insert(key, next_cost);
                        prev.push((-next_cost, key));
                    }
                }
            }
        }

        unreachable!()
    }

    #[test]
    fn part_01() {
        let input = include_str!("input.txt");

        let grid = input
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let result = dijkstra(grid, 1, 3);

        assert_eq!(result, 953);
    }

    #[test]
    fn part_02() {
        let input = include_str!("input.txt");

        let grid = input
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let result = dijkstra(grid, 4, 10);

        assert_eq!(result, 1180);
    }
}
