use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline},
    combinator::opt,
    multi::many1,
    IResult,
};

use crate::Network;

fn parse_directions(input: &str) -> IResult<&str, Vec<char>> {
    let (rem, chars) = many1(alt((char('L'), char('R'))))(input)?;
    let (rem, _) = opt(newline)(rem)?;

    Ok((rem, chars))
}

fn parse_edge(input: &str) -> IResult<&str, (String, (String, String))> {
    let (rem, start) = alpha1(input)?;
    let (rem, _) = tag(" = (")(rem)?;
    let (rem, left) = alpha1(rem)?;
    let (rem, _) = tag(", ")(rem)?;
    let (rem, right) = alpha1(rem)?;
    let (rem, _) = tag(")")(rem)?;
    let (rem, _) = opt(newline)(rem)?;

    Ok((
        rem,
        (start.to_owned(), (left.to_string(), right.to_string())),
    ))
}

pub fn parse_input(input: &str) -> IResult<&str, Network> {
    let (rem, directions) = parse_directions(input)?;
    let (rem, _) = newline(rem)?;
    let (rem, nodes) = many1(parse_edge)(rem)?;

    Ok((rem, Network::new(directions, nodes)))
}

#[cfg(test)]
mod tests {

    use crate::Network;

    use super::*;

    #[test]
    fn parse_directions_returns_directions() {
        assert_eq!(Ok(("", vec!['L', 'L', 'R'])), parse_directions("LLR"));
    }

    #[test]
    fn parse_locations_returns_locations() {
        assert_eq!(
            Ok((
                "",
                ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string()))
            )),
            parse_edge("AAA = (BBB, BBB)")
        );
    }

    #[test]
    fn parse_input_returns_structure() {
        assert_eq!(
            Ok((
                "",
                Network::new(
                    vec!['L', 'L', 'R'],
                    vec![
                        ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())),
                        ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string())),
                        ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string()))
                    ]
                )
            )),
            parse_input(&support::read_input_file_as_string("resource/small"))
        );
    }
}
