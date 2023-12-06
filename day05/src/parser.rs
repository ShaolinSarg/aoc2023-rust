use std::{collections::HashSet, vec};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map_res, opt},
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

use crate::RawMapping;

fn parse_seeds(input: &str) -> IResult<&str, HashSet<u32>> {
    let (i, _) = tag("seeds: ")(input)?;
    let (i, numbers) = separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u32>()))(i)?;
    let (i, _) = newline(i)?;

    Ok((i, numbers.into_iter().collect()))
}

fn parse_mapping_number(input: &str) -> IResult<&str, u32> {
    let (rem, number) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    Ok((rem, number))
}

fn parse_mappings<'a>(input: &'a str, mapping_tag: &'a str) -> IResult<&'a str, Vec<RawMapping>> {
    let (rem, _) = tag(mapping_tag)(input)?;
    let (rem, _) = newline(rem)?;
    let (rem, mappings) = many1(tuple((
        parse_mapping_number,
        space1,
        parse_mapping_number,
        space1,
        parse_mapping_number,
        opt(newline),
    )))(rem)?;

    let raw_mappings: Vec<RawMapping> = mappings
        .iter()
        .map(|&(s, _, d, _, r, _)| RawMapping::new(s, d, r))
        .collect();

    Ok((rem, raw_mappings))
}

pub fn parse_input(
    input: &str,
) -> IResult<
    &str,
    (
        HashSet<u32>,
        Vec<RawMapping>,
        Vec<RawMapping>,
        Vec<RawMapping>,
        Vec<RawMapping>,
        Vec<RawMapping>,
        Vec<RawMapping>,
        Vec<RawMapping>
    ),
> {
    let (rem, seeds) = parse_seeds(input)?;
    let (rem, _) = newline(rem)?;
    let (rem, seed_to_soil_map) = parse_mappings(rem, "seed-to-soil map:")?;
    let (rem, _) = newline(rem)?;
    let (rem, soil_to_fert_map) = parse_mappings(rem, "soil-to-fertilizer map:")?;
    let (rem, _) = newline(rem)?;
    let (rem, fert_to_water_map) = parse_mappings(rem, "fertilizer-to-water map:")?;
    let (rem, _) = newline(rem)?;
    let (rem, water_to_light_map) = parse_mappings(rem, "water-to-light map:")?;
    let (rem, _) = newline(rem)?;
    let (rem, light_to_temp_map) = parse_mappings(rem, "light-to-temperature map:")?;
    let (rem, _) = newline(rem)?;
    let (rem, temp_to_humidity_map) = parse_mappings(rem, "temperature-to-humidity map:")?;
    let (rem, _) = newline(rem)?;
    let (rem, humidity_to_locn_map) = parse_mappings(rem, "humidity-to-location map:")?;

    Ok((
        rem,
        (
            seeds,
            seed_to_soil_map,
            soil_to_fert_map,
            fert_to_water_map,
            water_to_light_map,
            light_to_temp_map,
            temp_to_humidity_map,
            humidity_to_locn_map
        ),
    ))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn parse_seeds_returns_set_of_seeds_to_plant() {
        let expected = Ok(("", HashSet::from([79, 14, 55, 13])));
        assert_eq!(expected, parse_seeds("seeds: 79 14 55 13\n"));
    }

    #[test]
    fn parse_mappings_returns_seed_to_soil_map() {
        assert_eq!(
            Ok(("", vec![RawMapping::new(50, 98, 2)])),
            parse_mappings("seed-to-soil map:\n50 98 2\n", "seed-to-soil map:")
        );

        assert_eq!(
            Ok((
                "",
                vec![RawMapping::new(50, 98, 2), RawMapping::new(52, 50, 48)]
            )),
            parse_mappings(
                "seed-to-soil map:\n50 98 2\n52 50 48\n",
                "seed-to-soil map:"
            )
        );
    }

    #[test]
    fn parse_input_returns_all_input() {
        let input = support::read_input_file_as_string("resource/small");

        assert_eq!(
            Ok((
                "",
                (
                    HashSet::from([79, 14, 55, 13]),
                    vec![
                        RawMapping::new(50, 98, 2), 
                        RawMapping::new(52, 50, 48)
                    ],
                    vec![
                        RawMapping::new(0, 15, 37),
                        RawMapping::new(37, 52, 2),
                        RawMapping::new(39, 0, 15)
                    ],
                    vec![
                        RawMapping::new(49, 53, 8),
                        RawMapping::new(0, 11, 42),
                        RawMapping::new(42, 0, 7),
                        RawMapping::new(57, 7, 4)
                    ],
                    vec![
                        RawMapping::new(88, 18, 7), 
                        RawMapping::new(18, 25, 70)
                    ],
                    vec![
                        RawMapping::new(45, 77, 23),
                        RawMapping::new(81, 45, 19),
                        RawMapping::new(68, 64, 13)
                    ],
                    vec![
                        RawMapping::new(0, 69, 1), 
                        RawMapping::new(1, 0, 69)
                    ],
                    vec![
                        RawMapping::new(60, 56, 37),
                        RawMapping::new(56, 93, 4)
                    ]
                )
            )),
            parse_input(&input)
        );
    }
}
