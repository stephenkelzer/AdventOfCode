#[cfg(test)]
mod day_07 {
    use itertools::Itertools;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
    enum HandType {
        FiveOfAKind,
        FourOfAKind,
        FullHouse,
        ThreeOfAKind,
        TwoPair,
        OnePair,
        HighCard,
    }

    #[derive(Debug)]
    struct Hand {
        cards: String,
        bid: usize,
        pub hand_type: HandType,
    }
    impl Hand {
        fn new((cards_str, bid): (&str, usize), jokers: bool) -> Self {
            if jokers && cards_str.contains('J') {
                return Hand {
                    cards: cards_str.to_string(),
                    bid,
                    hand_type: find_highest_joker_hand(cards_str),
                };
            }

            Hand {
                cards: cards_str.to_string(),
                bid,
                hand_type: get_hand_type(cards_str),
            }
        }
    }

    fn get_hand_type(cards_str: &str) -> HandType {
        let mut counts = HashMap::new();
        for c in cards_str.chars().sorted() {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        let mut counts_sorted = counts.values().collect_vec();
        counts_sorted.sort();

        match counts_sorted.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn find_highest_joker_hand(cards_str: &str) -> HandType {
        let cards_str = cards_str.replace('J', "");

        let sorted = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2"]
            .iter()
            .combinations_with_replacement(5 - cards_str.len())
            .map(|x| format!("{}{}", &cards_str, x.iter().join("")))
            .map(|cards_str| Hand::new((&cards_str, 0), false))
            .sorted_by(part_2_hand_sort);

        sorted
            .rev()
            .take(1) // this might need to be last instead of first?
            .next()
            .unwrap()
            .hand_type
    }

    fn part_2_hand_sort(hand_1: &Hand, hand_2: &Hand) -> std::cmp::Ordering {
        use HandType::*;

        let card_strength_map = {
            let mut map = HashMap::new();
            map.insert("A".to_string(), 13);
            map.insert("K".to_string(), 12);
            map.insert("Q".to_string(), 11);
            map.insert("T".to_string(), 10);
            map.insert("9".to_string(), 9);
            map.insert("8".to_string(), 8);
            map.insert("7".to_string(), 7);
            map.insert("6".to_string(), 6);
            map.insert("5".to_string(), 5);
            map.insert("4".to_string(), 4);
            map.insert("3".to_string(), 3);
            map.insert("2".to_string(), 2);
            map.insert("J".to_string(), 1);

            map
        };

        let equal_cmp = |a: &str, b: &str| -> std::cmp::Ordering {
            let mut i = 0;
            let mut result: Option<std::cmp::Ordering> = None;

            while result.is_none() && i <= a.len() {
                let a = a.chars().nth(i).unwrap();
                let b = b.chars().nth(i).unwrap();
                let a = card_strength_map.get(&a.to_string()).unwrap();
                let b = card_strength_map.get(&b.to_string()).unwrap();

                match a.cmp(b) {
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => result = Some(std::cmp::Ordering::Greater),
                    std::cmp::Ordering::Less => result = Some(std::cmp::Ordering::Less),
                }

                i += 1;
            }

            result.unwrap_or(std::cmp::Ordering::Equal)
        };

        match (hand_1.hand_type, hand_2.hand_type) {
            (FiveOfAKind, FiveOfAKind) => equal_cmp(&hand_1.cards, &hand_2.cards),
            (FiveOfAKind, _) => std::cmp::Ordering::Greater,
            (_, FiveOfAKind) => std::cmp::Ordering::Less,

            (FourOfAKind, FourOfAKind) => equal_cmp(&hand_1.cards, &hand_2.cards),
            (FourOfAKind, _) => std::cmp::Ordering::Greater,
            (_, FourOfAKind) => std::cmp::Ordering::Less,

            (FullHouse, FullHouse) => equal_cmp(&hand_1.cards, &hand_2.cards),
            (FullHouse, _) => std::cmp::Ordering::Greater,
            (_, FullHouse) => std::cmp::Ordering::Less,

            (ThreeOfAKind, ThreeOfAKind) => equal_cmp(&hand_1.cards, &hand_2.cards),
            (ThreeOfAKind, _) => std::cmp::Ordering::Greater,
            (_, ThreeOfAKind) => std::cmp::Ordering::Less,

            (TwoPair, TwoPair) => equal_cmp(&hand_1.cards, &hand_2.cards),
            (TwoPair, _) => std::cmp::Ordering::Greater,
            (_, TwoPair) => std::cmp::Ordering::Less,

            (OnePair, OnePair) => equal_cmp(&hand_1.cards, &hand_2.cards),
            (OnePair, _) => std::cmp::Ordering::Greater,
            (_, OnePair) => std::cmp::Ordering::Less,

            (HighCard, HighCard) => equal_cmp(&hand_1.cards, &hand_2.cards),
        }
    }

    #[test]
    fn part_01() {
        let card_strength_map = {
            let mut map = HashMap::new();
            map.insert("A".to_string(), 13);
            map.insert("K".to_string(), 12);
            map.insert("Q".to_string(), 11);
            map.insert("J".to_string(), 10);
            map.insert("T".to_string(), 9);
            map.insert("9".to_string(), 8);
            map.insert("8".to_string(), 7);
            map.insert("7".to_string(), 6);
            map.insert("6".to_string(), 5);
            map.insert("5".to_string(), 4);
            map.insert("4".to_string(), 3);
            map.insert("3".to_string(), 2);
            map.insert("2".to_string(), 1);

            map
        };

        let lines = include_str!("input.txt").lines();
        let mut hands: Vec<Hand> = lines
            .map(|l| l.split_once(' ').unwrap())
            .map(|(a, b)| Hand::new((a, b.parse::<usize>().unwrap()), false))
            .collect_vec();

        hands.sort_by(|hand_1, hand_2| {
            use HandType::*;

            let equal_cmp = |a: &str, b: &str| -> std::cmp::Ordering {
                let mut i = 0;
                let mut result: Option<std::cmp::Ordering> = None;

                while result.is_none() && i <= a.len() {
                    let a = a.chars().nth(i).unwrap();
                    let b = b.chars().nth(i).unwrap();
                    let a = card_strength_map.get(&a.to_string()).unwrap();
                    let b = card_strength_map.get(&b.to_string()).unwrap();

                    match a.cmp(b) {
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => result = Some(std::cmp::Ordering::Greater),
                        std::cmp::Ordering::Less => result = Some(std::cmp::Ordering::Less),
                    }

                    i += 1;
                }

                result.unwrap_or(std::cmp::Ordering::Equal)
            };

            match (hand_1.hand_type, hand_2.hand_type) {
                (FiveOfAKind, FiveOfAKind) => equal_cmp(&hand_1.cards, &hand_2.cards),
                (FiveOfAKind, _) => std::cmp::Ordering::Greater,
                (_, FiveOfAKind) => std::cmp::Ordering::Less,

                (FourOfAKind, FourOfAKind) => equal_cmp(&hand_1.cards, &hand_2.cards),
                (FourOfAKind, _) => std::cmp::Ordering::Greater,
                (_, FourOfAKind) => std::cmp::Ordering::Less,

                (FullHouse, FullHouse) => equal_cmp(&hand_1.cards, &hand_2.cards),
                (FullHouse, _) => std::cmp::Ordering::Greater,
                (_, FullHouse) => std::cmp::Ordering::Less,

                (ThreeOfAKind, ThreeOfAKind) => equal_cmp(&hand_1.cards, &hand_2.cards),
                (ThreeOfAKind, _) => std::cmp::Ordering::Greater,
                (_, ThreeOfAKind) => std::cmp::Ordering::Less,

                (TwoPair, TwoPair) => equal_cmp(&hand_1.cards, &hand_2.cards),
                (TwoPair, _) => std::cmp::Ordering::Greater,
                (_, TwoPair) => std::cmp::Ordering::Less,

                (OnePair, OnePair) => equal_cmp(&hand_1.cards, &hand_2.cards),
                (OnePair, _) => std::cmp::Ordering::Greater,
                (_, OnePair) => std::cmp::Ordering::Less,

                (HighCard, HighCard) => equal_cmp(&hand_1.cards, &hand_2.cards),
            }
        });

        let total_winnings = hands.iter().enumerate().fold(0, |acc, (i, hand)| {
            let winnings = hand.bid * (i + 1);
            acc + winnings
        });

        assert_eq!(total_winnings, 251136060);
    }

    #[test]
    fn part_02() {
        let lines = include_str!("input.txt").lines();
        let mut hands: Vec<Hand> = lines
            .map(|l| l.split_once(' ').unwrap())
            .map(|(a, b)| Hand::new((a, b.parse::<usize>().unwrap()), true))
            .collect_vec();

        hands.sort_by(part_2_hand_sort);

        let total_winnings = hands.iter().enumerate().fold(0, |acc, (i, hand)| {
            let winnings = hand.bid * (i + 1);
            acc + winnings
        });

        assert_eq!(total_winnings, 249400220);
    }
}
