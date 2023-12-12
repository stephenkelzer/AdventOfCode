#[cfg(test)]
mod day_12 {
    use itertools::Itertools;

    fn generate_permutations(input: &str) -> Vec<String> {
        // Convert the input string into characters
        let chars: Vec<char> = input.chars().collect();

        // Collect the indices of wildcard characters in the input string
        let wildcard_indices: Vec<usize> = chars
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c == '?' { Some(i) } else { None })
            .collect();

        // Generate all possible combinations of '#' and '.' for the wildcards
        let wildcard_combinations = (0..2u64.pow(wildcard_indices.len() as u32)).map(|x| {
            wildcard_indices
                .iter()
                .enumerate()
                .map(|(i, &index)| {
                    if x & (1 << i) == 0 {
                        ('#', index)
                    } else {
                        ('.', index)
                    }
                })
                .collect::<Vec<_>>()
        });

        // Replace the wildcards with the generated combinations and collect the results
        let result: Vec<String> = wildcard_combinations
            .map(|wildcard_combination| {
                let mut result_chars = chars.clone();
                for (c, index) in wildcard_combination {
                    result_chars[index] = c;
                }
                result_chars.iter().collect()
            })
            .collect();

        result
    }

    fn is_valid(permutation: &str, map: &Vec<usize>) -> bool {
        // Convert the input string into characters
        let chars: Vec<char> = permutation.chars().collect();

        // Track the lengths of consecutive '#' groups
        let mut group_lengths = Vec::new();
        let mut current_length = 0;

        // Iterate through the characters in the permutation
        for &c in &chars {
            if c == '#' {
                // Increment the length of the current '#' group
                current_length += 1;
            } else if current_length > 0 {
                // Add the length of the completed '#' group to the vector
                group_lengths.push(current_length);
                current_length = 0;
            }
        }

        // Add the length of the last '#' group if it exists
        if current_length > 0 {
            group_lengths.push(current_length);
        }

        // Check if the lengths of the '#' groups match the given map
        let matching = group_lengths
            .iter()
            .zip(map.iter())
            .filter(|&(a, b)| a == b)
            .count();

        matching == group_lengths.len() && matching == map.len()
    }

    #[test]
    fn part_01() {
        let answer = include_str!("input.txt")
            .lines()
            .map(|l| {
                let (a, b) = l.trim().split_once(" ").unwrap();
                (
                    a.trim(),
                    b.split(',')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect_vec(),
                )
            })
            .map(|r| {
                let possible_permutations = generate_permutations(r.0);
                let valid_permutations = possible_permutations.iter().filter(|p| is_valid(p, &r.1));
                valid_permutations.count()
            })
            .sum::<usize>();

        // println!("{:?}", generate_permutations(".??..??...?##."));

        assert_eq!(answer, 7857);
    }

    #[test]
    fn part_02() {
        assert_eq!(123, 123);
    }
}
