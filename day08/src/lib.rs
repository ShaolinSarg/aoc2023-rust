use std::collections::HashMap;

mod parser;

#[derive(PartialEq, Eq, Debug)]
pub struct Network {
    directions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

impl Network {
    fn new(directions: Vec<char>, nodes: Vec<(String, (String, String))>) -> Self {
        Network {
            directions,
            nodes: HashMap::from_iter(nodes),
        }
    }
}

pub fn day08_part1_answer(path: &str) -> String {
    let input = support::read_input_file_as_string(path);
    let network = parser::parse_input(&input).unwrap().1;

    let answer = count_moves(network);

    format!("{}", answer)
}

fn count_moves(network: Network) -> usize {
    let d = network.directions.iter().cycle();
    let mut current_location = "AAA";

    d.map(|&turn| {
        current_location = if turn == 'L' {
            &network.nodes.get(current_location).unwrap().0
        } else {
            &network.nodes.get(current_location).unwrap().1
        };
        current_location
    })
    .take_while(|&node| node != "ZZZ")
    .count()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let input = support::read_input_file_as_string("resource/small");
        let network = parse_input(&input).unwrap().1;

        assert_eq!(6, count_moves(network));
    }

    #[test]
    fn test_part1_answer() {
        let input = support::read_input_file_as_string("resource/input");
        let network = parse_input(&input).unwrap().1;

        assert_eq!(22199, count_moves(network));
    }
}
