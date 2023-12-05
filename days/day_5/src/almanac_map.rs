#[derive(Clone, Debug)]
pub struct MapRecord {
    pub source_range_start: usize,
    pub destination_range_start: usize,
    pub range_length: usize,
}

#[derive(Clone, Debug)]
pub struct AlmanacMap {
    pub source: String,
    pub destination: String,
    
    records: Vec<MapRecord>,
}

impl AlmanacMap {
    pub fn new(title_value: &str) -> Self {
        let cleaned_title_value = title_value
            .strip_suffix("map:")
            .expect("input did not contain the expected suffix of 'map:'")
            .trim();
        let (source, destination) = cleaned_title_value
            .split_once("-to-")
            .expect("failed to split map title");

        Self {
            source: source.to_string(),
            destination: destination.to_string(),
            records: vec![],
        }
    }

    pub fn parse_and_add_record(&mut self, record_value: &str) {
        let records = record_value
            .trim()
            .split(' ')
            .map(|x| x.parse::<usize>().expect("failed to parse record number"))
            .collect::<Vec<_>>();

        let destination_range_start = records[0];
        let source_range_start = records[1];
        let range_length = records[2];

        self.records.push(MapRecord {
            source_range_start,
            destination_range_start,
            range_length,
        });
    }

    pub fn get_destination(&self, source: usize) -> usize {
        self.records
            .iter()
            .find(|x| {
                x.source_range_start <= source && source < x.source_range_start + x.range_length
            })
            .map(|x| source - x.source_range_start + x.destination_range_start)
            .unwrap_or(source)
    }
}
