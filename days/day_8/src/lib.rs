#[cfg(test)]
mod day_8 {
    use itertools::Itertools;

    #[test]
    fn part_one() {
        let mut lines = include_str!("input.txt").lines();
        let instructions = lines.next().unwrap().chars().collect_vec();
        let instructions_length = instructions.len();

        // Vec<(curr_location, left_option, right_option)>
        let nodes = lines
            .skip(1)
            .map(|l| {
                let (location, options) = l.split_once(" = ").unwrap();
                let (l, r) = options.split_once(", ").unwrap();
                (location.to_string(), l.replace('(', ""), r.replace(')', ""))
            })
            .collect_vec();

        let mut steps_taken: usize = 0;
        let mut current_node = nodes
            .iter()
            .find(|(location, _, _)| location == "AAA")
            .unwrap();
        let mut made_it = false;

        while !made_it {
            let current_instruction = instructions[steps_taken % instructions_length];

            let next_node = nodes
                .iter()
                .find(|(location, _, _)| {
                    (current_instruction == 'L' && current_node.1.eq(location))
                        || (current_instruction == 'R' && current_node.2.eq(location))
                })
                .unwrap();

            current_node = next_node;
            steps_taken += 1;

            if next_node.0.eq("ZZZ") {
                made_it = true
            }
        }

        assert_eq!(steps_taken, 16043);
    }

    #[test]
    fn part_two() {
        // TODO

        assert_eq!(123, 123);
    }
}
