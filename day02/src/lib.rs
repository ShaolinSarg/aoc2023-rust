mod parser;

use std::{str::FromStr, collections::HashMap};

use support::read_input_file_as_lines;

#[derive(PartialEq, Eq, Debug, Hash)]
enum Colour {
    Blue,
    Green,
    Red,
}

impl FromStr for Colour {
    type Err = ();

    fn from_str(input: &str) -> Result<Colour, Self::Err> {
        match input {
            "blue" => Ok(Colour::Blue),
            "green" => Ok(Colour::Green),
            "red" => Ok(Colour::Red),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    id: u8,
    draws: Vec<HashMap<Colour, u8>>
}

pub fn day02_part1_answer(path: &str) -> String {
    format!("{}", "")
}


#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn parse_game_id_should_return_the_game_id() {
    //     let result = Ok(("", 1));
    //     assert_eq!(result, parse_game_id("Game 1:"));
    // }
}
