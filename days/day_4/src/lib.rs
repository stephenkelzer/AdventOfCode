#[cfg(test)]
mod day_4 {
    use std::{collections::HashMap, ops::Add};

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
        let mut current_line_number: usize = 0;
        let mut card_extra_copies_tracker = HashMap::<usize, usize>::new();

        let result = include_str!("cards.txt")
            .split("\n")
            .fold(0, |total_card_count, line| {
                current_line_number += 1;

                let current_card_instances = card_extra_copies_tracker
                    .get(&current_line_number)
                    .unwrap_or(&0)
                    .add(1); // copies plus original

                let (winning_numbers_string, player_numbers_string) =
                    line.split_once(" | ").unwrap();

                let winning_numbers = winning_numbers_string
                    .split_once(": ")
                    .unwrap()
                    .1
                    .split(' ')
                    .filter_map(|x| match x.is_empty() {
                        true => None,
                        false => Some(x.parse::<usize>().unwrap()),
                    })
                    .collect::<Vec<_>>();

                let card_win_count = player_numbers_string
                    .split(' ')
                    .filter_map(|curr| {
                        if let Ok(curr) = curr.trim().parse::<usize>() {
                            if winning_numbers.contains(&curr) {
                                return Some(1);
                            }
                        }

                        None
                    })
                    .count();

                if card_win_count > 0 {
                    // if there were any winning numbers, add appropriate copies of following cards
                    for n in (current_line_number + 1)..=(current_line_number + card_win_count) {
                        let curr_val = card_extra_copies_tracker.get(&n).unwrap_or(&0);
                        card_extra_copies_tracker.insert(n, curr_val + current_card_instances);
                    }
                }

                card_extra_copies_tracker.remove(&current_line_number);

                total_card_count + current_card_instances
            });

        assert_eq!(result, 19499881);
    }
}
