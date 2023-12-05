#[cfg(test)]
mod almanac_map;

#[cfg(test)]
mod config;

#[cfg(test)]
mod day_5 {
    use crate::config::Configuration;

    #[test]
    fn part_one() {
        let config = Configuration::default();
        println!("config: {:?}", config);

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
    fn part_two() {
        assert_eq!(123, 123);
    }
}
