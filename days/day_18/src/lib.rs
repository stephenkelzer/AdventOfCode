#[cfg(test)]
mod day_18 {
    use itertools::Itertools;

    fn rotate_cw(dir: char) -> char {
        match dir {
            'U' => 'R',
            'R' => 'D',
            'D' => 'L',
            'L' => 'U',
            _ => unreachable!(),
        }
    }

    fn rotate_ccw(dir: char) -> char {
        match dir {
            'U' => 'L',
            'L' => 'D',
            'D' => 'R',
            'R' => 'U',
            _ => unreachable!(),
        }
    }

    #[derive(Clone, Copy)]
    struct Instruction {
        direction: char, // U, D, L, R
        distance: usize,
    }

    fn find_lagoon_area(instructions: &[Instruction]) -> usize {
        let clockwise = instructions
            .windows(2)
            .fold(0, |mut acc: isize, w| {
                if rotate_cw(w[0].direction) == w[1].direction {
                    acc += 1;
                } else if rotate_ccw(w[0].direction) == w[1].direction {
                    acc -= 1;
                } else {
                    unreachable!()
                }

                acc
            })
            .is_positive();
        let mut area: isize = 0;
        let mut pos: (isize, isize) = (0, 0);

        let corner = |t1: Instruction, t2: Instruction, pos: (isize, isize)| {
            if clockwise {
                match (t1.direction, t2.direction) {
                    ('U', 'R') | ('R', 'U') => pos,
                    ('U', 'L') | ('L', 'U') => (pos.0, pos.1 + 1),
                    ('R', 'D') | ('D', 'R') => (pos.0 + 1, pos.1),
                    ('L', 'D') | ('D', 'L') => (pos.0 + 1, pos.1 + 1),
                    _ => unreachable!(),
                }
            } else {
                match (t1.direction, t2.direction) {
                    ('U', 'R') | ('R', 'U') => (pos.0 + 1, pos.1 + 1),
                    ('U', 'L') | ('L', 'U') => (pos.0 + 1, pos.1),
                    ('R', 'D') | ('D', 'R') => (pos.0, pos.1 + 1),
                    ('L', 'D') | ('D', 'L') => pos,
                    _ => unreachable!(),
                }
            }
        };

        for i in 0..instructions.len() {
            let prev = instructions[(i + instructions.len() - 1) % instructions.len()];
            let cur = instructions[i];
            let next = instructions[(i + 1) % instructions.len()];

            let (x_i, y_i) = corner(prev, cur, pos);
            match cur.direction {
                'U' => pos.1 -= cur.distance as isize,
                'D' => pos.1 += cur.distance as isize,
                'L' => pos.0 -= cur.distance as isize,
                'R' => pos.0 += cur.distance as isize,
                _ => unreachable!(),
            }
            let (x_j, y_j) = corner(cur, next, pos);

            area += x_i * y_j - x_j * y_i;
        }

        area.unsigned_abs() / 2
    }

    #[test]
    fn part_01() {
        let instructions = include_str!("input.txt")
            .lines()
            .map(|l| {
                let (direction, tail) = l.split_once(' ').unwrap();
                let (distance, _) = tail.split_once(' ').unwrap();

                Instruction {
                    direction: direction.parse::<char>().unwrap(),
                    distance: distance.parse::<usize>().unwrap(),
                }
            })
            .collect_vec();

        let answer = find_lagoon_area(&instructions);

        assert_eq!(answer, 62365);
    }

    #[test]
    fn part_02() {
        let instructions = include_str!("input.txt")
            .lines()
            .map(|l| {
                let color = l
                    .split_terminator(&['(', ')'])
                    .nth(1)
                    .unwrap()
                    .replace("#", "");

                let r = usize::from_str_radix(&color[0..2], 16).unwrap();
                let g = usize::from_str_radix(&color[2..4], 16).unwrap();
                let b = usize::from_str_radix(&color[4..6], 16).unwrap();

                let direction = match b & 0x0F {
                    0 => 'U',
                    1 => 'L',
                    2 => 'D',
                    3 => 'R',
                    _ => unreachable!(),
                };

                Instruction {
                    direction,
                    distance: (r << 12) | (g << 4) | (b >> 4),
                }
            })
            .collect::<Vec<_>>();

        let answer = find_lagoon_area(&instructions);
        assert_eq!(answer, 159485361249806);
    }
}
