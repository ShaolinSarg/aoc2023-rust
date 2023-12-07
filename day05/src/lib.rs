use parser::parse_input;
use support::read_input_file_as_string;

mod parser;

pub fn day05_part1_answer(path: &str) -> String {
    let input = read_input_file_as_string(path);
    let (
        seeds,
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humid,
        humid_to_locn,
    ) = parse_input(&input).unwrap().1;

    let locations: Vec<u64> = seeds
        .iter()
        .map(|&s| {
            let soil = map_ranges(s, &seed_to_soil);
            let fert = map_ranges(soil, &soil_to_fert);
            let water = map_ranges(fert, &fert_to_water);
            let light = map_ranges(water, &water_to_light);
            let temp = map_ranges(light, &light_to_temp);
            let hum = map_ranges(temp, &temp_to_humid);
            map_ranges(hum, &humid_to_locn)
        })
        .collect();

    let answer = locations.iter().min().unwrap();

    format!("{}", answer)
}

#[derive(PartialEq, Eq, Debug)]
pub struct RawMapping {
    dest_start: u64,
    source_start: u64,
    range_length: u64,
}
impl RawMapping {
    fn new(d: u64, s: u64, l: u64) -> Self {
        RawMapping {
            dest_start: d,
            source_start: s,
            range_length: l,
        }
    }
}

fn map_range(key: u64, mapping: &RawMapping) -> Option<u64> {
    let mapping_upper = mapping.source_start + mapping.range_length;

    if mapping.source_start <= key && key < mapping_upper {
        let difference = key - mapping.source_start;
        let dest_value = mapping.dest_start + difference;

        Some(dest_value)
    } else {
        None
    }
}

fn map_ranges(key: u64, mappings: &[RawMapping]) -> u64 {
    mappings
        .iter()
        .find_map(|m| map_range(key, m))
        .unwrap_or(key)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn map_range_should_return_key_if_in_range() {
        assert_eq!(Some(50), map_range(98, &RawMapping::new(50, 98, 2)));
        assert_eq!(Some(51), map_range(99, &RawMapping::new(50, 98, 2)));
        assert_eq!(None, map_range(97, &RawMapping::new(50, 98, 2)));
        assert_eq!(None, map_range(100, &RawMapping::new(50, 98, 2)));
        assert_eq!(Some(81), map_range(79, &RawMapping::new(52, 50, 48)));
    }

    #[test]
    fn map_ranges_should_return_mapped_key() {
        let mappings = vec![RawMapping::new(50, 98, 2), RawMapping::new(52, 50, 48)];

        assert_eq!(81, map_ranges(79, &mappings));
        assert_eq!(14, map_ranges(14, &mappings));
        assert_eq!(57, map_ranges(55, &mappings));
        assert_eq!(13, map_ranges(13, &mappings));
    }

    #[test]
    fn seed_to_locn_returns_location_from_seed() {
        assert_eq!("35", day05_part1_answer("resource/small"));
    }
}
