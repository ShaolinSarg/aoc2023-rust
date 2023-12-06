use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1, newline},
    combinator::map_res,
    multi::{separated_list1, many1},
    IResult, sequence::tuple,
};

use crate::RawMapping;

fn parse_seeds(input: &str) -> IResult<&str, HashSet<u32>> {
    let (i, _) = tag("seeds: ")(input)?;
    let (i, numbers) = separated_list1(space1, map_res(digit1, |s: &str| s.parse::<u32>()))(i)?;
    let (i, _) = newline(i)?;
    
    Ok((i, numbers.into_iter().collect()))
}

fn parse_mapping_number(input: &str) -> IResult<&str, u32> {
    let (rem, number) = map_res(digit1, |s:&str| s.parse::<u32>())(input)?;
    Ok((rem, number))
}

fn parse_seed_to_soil_map(input: &str) -> IResult<&str, Vec<RawMapping>> {
    let (rem, _) = tag("seed-to-soil map:")(input)?;
    let (rem, _) = newline(rem)?;
    let (rem, mappings) = many1(tuple((
        parse_mapping_number,
        space1,
        parse_mapping_number,
        space1,
        parse_mapping_number,
        newline))
    )(rem)?;

    let raw_mappings:Vec<RawMapping> = mappings.iter().map(|&(s, _, d, _, r, _)| RawMapping::new(s, d, r)).collect();

    Ok((rem, raw_mappings))
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
    fn parse_seed_to_soil_map_returns_seed_to_soil_map() {
        assert_eq!(
            Ok(("", vec![RawMapping::new(50, 98, 2)])), 
            parse_seed_to_soil_map("seed-to-soil map:\n50 98 2\n")
        );

        assert_eq!(
            Ok(("", vec![RawMapping::new(50, 98, 2), RawMapping::new(52, 50, 48)])), 
            parse_seed_to_soil_map("seed-to-soil map:\n50 98 2\n52 50 48\n")
        );
    }

    //destination range start, the source range start, and the range length

}
