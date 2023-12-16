#[cfg(test)]
mod day_15 {
    use itertools::Itertools;
    use std::collections::BTreeMap;

    enum Operation {
        Set(String, u32, usize), // label, hash, focal_length
        Remove(String, u32),     // label, hash
    }

    fn hash(input: std::str::Chars<'_>) -> u32 {
        input
            .map(|x| x as u32)
            .fold(0, |total, c| ((total + c) * 17) % 256)
    }

    #[test]
    fn part_01() {
        let answer = include_str!("input.txt")
            .trim()
            .replace('\n', "")
            .split(',')
            .map(|step| hash(step.chars()) as usize)
            .sum::<usize>();

        assert_eq!(answer, 494980);
    }

    #[test]
    fn part_02() {
        let answer = include_str!("input.txt")
            .trim()
            .replace('\n', "")
            .split(',')
            .map(|x| {
                if x.contains('=') {
                    let (label, value) = x.split_once('=').unwrap();
                    return Operation::Set(
                        label.to_owned(),
                        hash(label.chars()),
                        value.parse::<usize>().unwrap(),
                    );
                }

                let (label, _) = x.split_once('-').unwrap();
                return Operation::Remove(label.to_owned(), hash(label.chars()));
            })
            .fold(
                BTreeMap::<u32, Vec<(String, usize)>>::new(),
                |mut map, curr| {
                    match curr {
                        Operation::Set(label, hash, focal_length) => match map.get_mut(&hash) {
                            Some(lens_map) => {
                                if let Some(lens) = lens_map.iter_mut().find(|x| x.0 == label) {
                                    lens.1 = focal_length;
                                } else {
                                    lens_map.push((label, focal_length));
                                }
                            }
                            None => {
                                map.insert(hash, vec![(label, focal_length)]);
                            }
                        },
                        Operation::Remove(label, hash) => {
                            if let Some(lens_map) = map.get_mut(&hash) {
                                if let Some((index, _)) =
                                    lens_map.iter().find_position(|x| x.0 == label)
                                {
                                    if lens_map.len() == 1 {
                                        map.remove(&hash);
                                    } else {
                                        lens_map.remove(index);
                                    }
                                }
                            }
                        }
                    }

                    map
                },
            )
            .iter()
            .map(|(box_number, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(|(i, (_, focal_length))| {
                        ((*box_number as usize + 1) * (i + 1)) * focal_length
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

        assert_eq!(answer, 247933);
    }
}
