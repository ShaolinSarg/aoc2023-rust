mod parser;

use std::{collections::HashMap, str::FromStr};

use support::read_input_file_as_lines;

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Colour {
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
pub struct Game {
    id: u32,
    draws: Vec<HashMap<Colour, u8>>,
}

pub fn day02_part1_answer(path: &str) -> String {
    let input_lines = read_input_file_as_lines(path);
    let games = input_lines.iter().map(|g| parser::parse_game(g).unwrap().1);

    let valid_games = games.into_iter().filter(game_is_valid);

    let answer: u32 = valid_games.map(|g| g.id).sum();

    format!("{}", answer)
}

fn game_is_valid(game: &Game) -> bool {
    game.draws.iter().all(is_valid_colour)
}

fn is_valid_colour(draws: &HashMap<Colour, u8>) -> bool {
    let blue_count = draws.get(&Colour::Blue).unwrap_or(&0);
    let green_count = draws.get(&Colour::Green).unwrap_or(&0);
    let red_count = draws.get(&Colour::Red).unwrap_or(&0);

    *blue_count <= 14 && *green_count <= 13 && *red_count <= 12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_is_valid_should_return_true_for_valid_games() {
        let game = Game {
            id: 1,
            draws: vec![
                HashMap::from([(Colour::Blue, 3), (Colour::Red, 4)]),
                HashMap::from([(Colour::Blue, 6), (Colour::Red, 1), (Colour::Green, 2)]),
                HashMap::from([(Colour::Green, 2)]),
            ],
        };
        assert_eq!(true, game_is_valid(&game));
    }

    #[test]
    fn game_is_valid_should_return_false_for_valid_games() {
        let game = Game {
            id: 3,
            draws: vec![
                HashMap::from([(Colour::Blue, 6), (Colour::Red, 20), (Colour::Green, 8)]),
                HashMap::from([(Colour::Blue, 5), (Colour::Red, 4), (Colour::Green, 13)]),
                HashMap::from([(Colour::Red, 1), (Colour::Green, 5)]),
            ],
        };
        assert_eq!(false, game_is_valid(&game));
    }

    #[test]
    fn part_1_should_return_answer() {
        assert_eq!("8", day02_part1_answer("resource/day02_small"));
    }
}
