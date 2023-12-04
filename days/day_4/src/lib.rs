#[cfg(test)]
mod day_4 {
    #[test]
    fn part_one() {
        let result = include_str!("cards.txt")
            .split("\n")
            .map(|line| {
                let splits = &line[line.trim().char_indices().nth(10).unwrap().0..]
                    .split(" | ")
                    .collect::<Vec<_>>();

                let winning_numbers = splits
                    .first()
                    .unwrap()
                    .split(' ')
                    .filter_map(|x| match x.is_empty() {
                        true => None,
                        false => Some(x.parse::<usize>().unwrap()),
                    })
                    .collect::<Vec<_>>();

                let card_score =
                    splits.last().unwrap().split(' ').fold(0, |acc, curr| {
                        match curr.trim().parse::<usize>() {
                            Ok(curr_number) => {
                                let mut score = acc;

                                if winning_numbers.contains(&curr_number) {
                                    if score == 0 {
                                        score = 1;
                                    } else {
                                        score = score * 2;
                                    }
                                }

                                score
                            }
                            _ => acc,
                        }
                    });

                card_score
            })
            .sum::<usize>();

        assert_eq!(result, 25651);
    }

    #[test]
    fn part_two() {
        // TODO

        let result: usize = 123;
        assert_eq!(result, 123);
    }
}
