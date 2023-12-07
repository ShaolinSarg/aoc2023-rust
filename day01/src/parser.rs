use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    multi::many1,
    IResult,
};

fn map_word_to_num(word: &str) -> &str {
    let m: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    m.get(word).unwrap_or(&word)
}

pub fn parse_line(input: &str) -> IResult<&str, String> {
    let (rem, nums) = many1(alt((parse_number, parse_number_word, parse_nonsense)))(input)?;

    let numbers: String = nums
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| map_word_to_num(s))
        .collect::<String>();

    Ok((rem, numbers))
}

fn parse_number(input: &str) -> IResult<&str, &str> {
    let (rem, number) = digit1(input)?;

    Ok((rem, number))
}

fn parse_number_word(input: &str) -> IResult<&str, &str> {
    let (rem, number) = alt((
        tag("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
    ))(input)?;

    Ok((rem, number))
}

fn parse_nonsense(input: &str) -> IResult<&str, &str> {
    let (rem, _) = anychar(input)?;

    Ok((rem, ""))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_digits_test() {
        assert_eq!(Ok(("", String::from("219"))), parse_line("two1nine"));
        assert_eq!(Ok(("", String::from("83"))), parse_line("eightwothree"));
        assert_eq!(Ok(("", String::from("123"))), parse_line("abcone2threexyz"));
        assert_eq!(Ok(("", String::from("234"))), parse_line("xtwone3four"));
        assert_eq!(
            Ok(("", String::from("49872"))),
            parse_line("4nineeightseven2")
        );
        assert_eq!(Ok(("", String::from("1234"))), parse_line("zoneight234"));
        assert_eq!(Ok(("", String::from("76"))), parse_line("7pqrstsixteen"));
        assert_eq!(
            Ok(("", String::from("8698"))),
            parse_line("eighttkbtzjz6nineeight")
        );
    }
}
