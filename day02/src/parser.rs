use std::{collections::HashMap, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    multi::separated_list1,
    sequence::Tuple,
    IResult,
};

use crate::{Colour, Game};

fn parse_game_id(input: &str) -> IResult<&str, u32> {
    let (i, _) = tag("Game ")(input)?;
    let (i, game_id) = digit1(i)?;
    let (i, _) = tag(": ")(i)?;

    Ok((i, game_id.parse::<u32>().unwrap()))
}

fn parse_colour_draw(input: &str) -> IResult<&str, (Colour, u8)> {
    let (i, count) = digit1(input)?;
    let (i, _) = space1(i)?;
    let (i, colour) = alt((tag("blue"), tag("green"), tag("red")))(i)?;

    Ok((
        i,
        (
            Colour::from_str(colour).unwrap(),
            count.parse::<u8>().unwrap(),
        ),
    ))
}

fn parse_draws(input: &str) -> IResult<&str, HashMap<Colour, u8>> {
    let (i, draws) = separated_list1(tag(", "), parse_colour_draw)(input)?;

    let m: HashMap<_, _> = draws.into_iter().collect();

    Ok((i, m))
}

fn parse_multi_draws(input: &str) -> IResult<&str, Vec<HashMap<Colour, u8>>> {
    let (i, all_draws) = separated_list1(tag("; "), parse_draws)(input)?;

    Ok((i, all_draws))
}
pub fn parse_game(input: &str) -> IResult<&str, Game> {
    let (i, (id, draws)) = (parse_game_id, parse_multi_draws).parse(input)?;

    Ok((i, Game { id, draws }))
}

#[cfg(test)]
mod tests {
    use crate::Game;

    use super::*;

    #[test]
    fn parse_game_id_should_return_the_game_id() {
        assert_eq!(Ok(("", 1)), parse_game_id("Game 1: "));
        assert_eq!(Ok(("", 31)), parse_game_id("Game 31: "));
    }

    #[test]
    fn parse_colour_draw_should_return_the_colour_and_count() {
        assert_eq!(Ok(("", (Colour::Blue, 3))), parse_colour_draw("3 blue"));
        assert_eq!(Ok(("", (Colour::Green, 2))), parse_colour_draw("2 green"));
    }

    #[test]
    fn parse_draws_should_return_multiple_colour_and_counts() {
        assert_eq!(
            Ok(("", HashMap::from([(Colour::Blue, 3), (Colour::Red, 2)]))),
            parse_draws("3 blue, 2 red")
        );
        assert_eq!(
            Ok(("", HashMap::from([(Colour::Red, 2)]))),
            parse_draws("2 red")
        );
    }

    #[test]
    fn parse_multi_draws_should_return_multiple_colour_and_counts() {
        assert_eq!(
            Ok((
                "",
                vec![
                    HashMap::from([(Colour::Blue, 3), (Colour::Red, 2)]),
                    HashMap::from([(Colour::Green, 12)])
                ]
            )),
            parse_multi_draws("3 blue, 2 red; 12 green")
        );
        assert_eq!(
            Ok(("", vec![HashMap::from([(Colour::Red, 2)])])),
            parse_multi_draws("2 red")
        );
    }

    #[test]
    fn parse_game_should_return_multiple_games() {
        let game_1_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(
            Ok((
                "",
                Game {
                    id: 1,
                    draws: vec![
                        HashMap::from([(Colour::Blue, 3), (Colour::Red, 4)]),
                        HashMap::from([(Colour::Blue, 6), (Colour::Red, 1), (Colour::Green, 2)]),
                        HashMap::from([(Colour::Green, 2)])
                    ]
                }
            )),
            parse_game(game_1_input)
        );

        let game_2_input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(
            Ok((
                "",
                Game {
                    id: 2,
                    draws: vec![
                        HashMap::from([(Colour::Blue, 1), (Colour::Green, 2)]),
                        HashMap::from([(Colour::Blue, 4), (Colour::Red, 1), (Colour::Green, 3)]),
                        HashMap::from([(Colour::Blue, 1), (Colour::Green, 1)])
                    ]
                }
            )),
            parse_game(game_2_input)
        );

        let game_3_input =
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(
            Ok((
                "",
                Game {
                    id: 3,
                    draws: vec![
                        HashMap::from([(Colour::Blue, 6), (Colour::Red, 20), (Colour::Green, 8)]),
                        HashMap::from([(Colour::Blue, 5), (Colour::Red, 4), (Colour::Green, 13)]),
                        HashMap::from([(Colour::Red, 1), (Colour::Green, 5)])
                    ]
                }
            )),
            parse_game(game_3_input)
        );
        // "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        // "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    }
}
