use crate::almanac_map::AlmanacMap;

#[derive(Debug)]
pub struct Configuration {
    pub seeds: Vec<usize>,
    pub maps: Vec<AlmanacMap>,
}

impl Default for Configuration {
    fn default() -> Self {
        let mut lines = include_str!("input.txt").lines();
        let first_line = lines.nth(0).unwrap();
        let seeds = first_line
            .replace("seeds: ", "")
            .split(" ")
            .map(|x| x.parse::<usize>().expect("failed to parse seed number"))
            .collect::<Vec<_>>();

        let mut maps = vec![];
        let mut current_map: Option<AlmanacMap> = None;

        lines.skip(1).for_each(|line| {
            match line.trim() {
                line if line.ends_with("map:") => {
                    // NEW MAP
                    current_map = Some(AlmanacMap::new(line));
                }
                line if line.is_empty() => {
                    // END OF MAP
                    maps.push(current_map.as_mut().expect("No current map!").clone());
                    current_map = None;
                }
                line => {
                    // VALUE IN CURRENT MAP
                    current_map
                        .as_mut()
                        .expect("No current map!")
                        .parse_and_add_record(line);
                }
            };
        });

        if let Some(current_map) = current_map {
            // catch hanging map at end of file
            maps.push(current_map);
        }

        Self { seeds, maps }
    }
}
