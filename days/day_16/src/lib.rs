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

        let is_valid_spot = |r: &isize, c: &isize| -> bool {
            // println!("checking: {}|{}|{}|{}", r, c, grid.len(), grid[0].len());
            r >= &0 && c >= &0 && r < &height && c < &width
        };

        let get_spot = |r: isize, c: isize| -> Option<char> {
            if is_valid_spot(&r, &c) {
                // println!("is_valid: {}|{}|{}|{}", r, c, grid.len(), grid[0].len());
                return Some(grid[r as usize][c as usize]);
            }

            None
        };

        let mut beams_queue = vec![Pos(0, 0, Dir::E)];

        // println!("l: {}", beams_queue.len());
        // beams_queue.push_back(Pos(1, 1, Dir::N));
        // beams_queue.push_back(Pos(1, 1, Dir::N));
        // beams_queue.push_back(Pos(1, 1, Dir::N));
        // println!("l: {}", beams_queue.len());
        // let _ = beams_queue.pop_back();
        // let _ = beams_queue.pop_back();
        // let _ = beams_queue.pop_back();
        // println!("l: {}", beams_queue.len());
        while beams_queue.len() > 0 {
            let beams_to_process = beams_queue
                .iter()
                .cloned()
                .filter(|x| {
                    if cache.contains_key(x) {
                        // println!("CACHE HIT");
                        return false;
                    }
                    true
                })
                .collect_vec();
            beams_queue.clear();

            // println!("group: {:?}", beams_to_process);
            // while let Some(__beam) = beams_queue.pop_front() {

            let next = beams_to_process
                .iter()
                .flat_map(|__beam| {
                    let Pos(r, c, heading) = *__beam;

                    // println!("len: {}", beams_queue.len());

                    // if beams_to_process.len() > 2 {
                    //     panic!("woah")
                    // }

                    if let Some(ch) = get_spot(r, c) {
                        // println!("PROCESSING: {}|{}|{:?}|{}", r, c, heading, ch);
                        // println!("valid: {}|{}", r, c);
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
                            // .inspect(|x| println!("going: {:?}", x))
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
                        // println!("PROCESSING: {}|{}|{:?}   skipping!", r, c, heading);
                        vec![]
                    }
                    // beams_queue
                    //     .iter()
                    //     .map(|x| (x.0, x.1, x.2))
                    //     .for_each(|x| println!("next: {x:?}"));

                    // println!(
                    //     "len: {}",
                    //     beams_queue
                    //         .iter()
                    //         .map(|x| (x.0, x.1, x.2))
                    //         .inspect(|x| println!("in: {x}"))
                    //         .collect_vec()
                    // );
                })
                .collect_vec();
            // println!("NEXT: {:?}", next);
            next.iter().for_each(|x| beams_queue.push(*x));
        }

        assert_eq!(energized_tiles.len(), 7477);
    }

    #[test]
    fn part_02() {
        let _input = include_str!("input.txt");

        assert_eq!(123, 123);
    }
}
