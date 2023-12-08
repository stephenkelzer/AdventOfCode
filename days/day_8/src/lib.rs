#[cfg(test)]
mod day_8 {
    use itertools::Itertools;
    use std::collections::HashMap;

    #[test]
    fn part_one() {
        let mut lines = include_str!("input.txt").lines();
        let instructions = lines.next().unwrap().chars().collect_vec();
        let instructions_length = instructions.len();

        let nodes: HashMap<String, (String, bool)> =
            HashMap::from_iter(lines.skip(1).flat_map(|l| {
                let (location, options) = l.split_once(" = ").unwrap();
                let (l, r) = options.split_once(", ").unwrap();
                let l = l.replace('(', "");
                let r = r.replace(')', "");

                vec![
                    (format!("{}-L", location), (l.to_string(), l == "ZZZ")),
                    (format!("{}-R", location), (r.to_string(), r == "ZZZ")),
                ]
            }));

        let mut steps_taken: usize = 0;
        let mut current_location = "AAA";
        let mut made_it = false;

        while !made_it {
            let current_instruction = instructions[steps_taken % instructions_length];

            let (next, is_z) = nodes
                .get(&format!("{current_location}-{current_instruction}"))
                .unwrap();

            current_location = next;
            steps_taken += 1;

            if *is_z {
                made_it = true
            }
        }

        assert_eq!(steps_taken, 16043);
    }

    #[test]
    fn part_two() {
        let mut lines = include_str!("input.txt").lines();
        let mut instructions = lines
            .next()
            .unwrap()
            .chars()
            .map(|x| if x.eq(&'L') { 0 } else { 1 })
            .cycle();

        let map: HashMap<String, [String; 2]> = HashMap::from_iter(lines.skip(1).map(|l| {
            let (location, options) = l.split_once(" = ").unwrap();
            let (l, r) = options.split_once(", ").unwrap();
            let l = l.replace('(', "");
            let r = r.replace(')', "");

            (location.to_string(), [l.to_string(), r.to_string()])
        }));

        let mut positions = map
            .iter()
            .filter_map(|(key, _)| {
                if key.ends_with('A') {
                    Some((key.as_str(), 0usize))
                } else {
                    None
                }
            })
            .collect_vec();

        let pos_len = positions.len();

        let mut answer = || -> usize {
            loop {
                let next_instruction = instructions.next().unwrap();

                for p in positions.iter_mut().filter(|(pos, _)| !pos.ends_with("Z")) {
                    p.0 = &map[p.0][next_instruction];
                    p.1 += 1;
                }

                if positions
                    .iter()
                    .filter(|(pos, _)| pos.ends_with('Z'))
                    .count()
                    == pos_len
                {
                    return positions
                        .iter()
                        .fold(1, |acc, (_, steps)| num::integer::lcm(acc, *steps));
                }
            }
        };

        assert_eq!(answer(), 15726453850399);
    }
}
