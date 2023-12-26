#[cfg(test)]
mod day_25 {
    use rand::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn part_01() {
        let mut rng = rand::thread_rng();
        let mut edges = vec![];
        for line in include_str!("input.txt").lines() {
            let (src, r) = line.split_once(": ").unwrap();
            for dst in r.split(' ') {
                edges.push((src, dst));
            }
        }

        let num_nodes = edges
            .iter()
            .flat_map(|(s, d)| [s, d])
            .collect::<HashSet<_>>()
            .len();
        let (mut l, mut r) = (HashSet::new(), HashSet::new());

        let answer: Option<usize>;

        'again: loop {
            let mut edges = edges.clone();
            edges.shuffle(&mut rng);
            l.clear();
            r.clear();

            for set in [&mut l, &mut r] {
                let (s, d) = edges.pop().unwrap();
                set.insert(s);
                set.insert(d);
            }

            while !edges.is_empty() {
                let mut i = 0;
                while i < edges.len() {
                    let (src, dst) = edges[i];
                    if l.contains(&src) {
                        if r.contains(&dst) {
                            continue 'again;
                        }
                        l.insert(dst);
                    } else if r.contains(&src) {
                        if l.contains(&dst) {
                            continue 'again;
                        }
                        r.insert(dst);
                    } else if l.contains(&dst) {
                        if r.contains(&src) {
                            continue 'again;
                        }
                        l.insert(src);
                    } else if r.contains(&dst) {
                        if l.contains(&src) {
                            continue 'again;
                        }
                        r.insert(src);
                    } else {
                        i += 1;
                        continue;
                    }

                    edges.swap_remove(i);

                    if !l.is_empty() && !r.is_empty() && l.len() + r.len() == num_nodes {
                        let bridges = edges
                            .iter()
                            .filter(|(s, d)| {
                                l.contains(s) && r.contains(d) || l.contains(d) && r.contains(s)
                            })
                            .count();
                        if bridges == 3 {
                            answer = Some(l.len() * r.len());
                            break 'again;
                        }
                    }
                }
            }
        }

        assert_eq!(answer.unwrap(), 555702);
    }

    #[test]
    fn part_02() {
        // There is no part 2 for day 25
        assert!(true);
    }
}
