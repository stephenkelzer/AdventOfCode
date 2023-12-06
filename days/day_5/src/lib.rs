#[cfg(test)]
mod almanac_map;

#[cfg(test)]
mod config;

#[cfg(test)]
mod day_5 {
    use crate::config::Configuration;
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    #[test]
    fn part_one() {
        let config = Configuration::default();

        let closest_location = config
            .seeds
            .iter()
            .map(|seed| {
                let mut current_value = seed.clone();
                let mut current_source = "seed";

                while current_source != "location" {
                    let mapper = config
                        .maps
                        .iter()
                        .find(|m| m.source == current_source)
                        .expect("No map found!");

                    current_value = mapper.get_destination(current_value);
                    current_source = mapper.destination.as_str();
                }

                current_value
            })
            .min()
            .expect("No min seed found!")
            .clone();

        assert_eq!(closest_location, 227653707);
    }

    #[test]
    #[ignore]// ignored because this test takes wayyyyy too long to run (at the moment!)
    fn part_two() {
        let config = Configuration::default();

        let closest_location = config
            .seeds
            .chunks(2)
            .filter_map(|s| {
                (s[0]..(s[0] + s[1]))
                    .into_par_iter()
                    .map(|seed| {
                        let mut current_value = seed.clone();
                        let mut current_source = "seed";

                        while current_source != "location" {
                            let mapper = config
                                .maps
                                .iter()
                                .find(|m| m.source == current_source)
                                .expect("No map found!");

                            current_value = mapper.get_destination(current_value);
                            current_source = mapper.destination.as_str();
                        }

                        current_value
                    })
                    .min()
            })
            .min().expect("No min seed found!");

        assert_eq!(closest_location, 78775051);
    }
}
