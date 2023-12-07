use std::collections::HashSet;
use support::read_input_file_as_lines;

pub fn day03_part1_answer(path: &str) -> String {
    let input_lines = read_input_file_as_lines(path);
    let all_symbols: HashSet<(i32, i32)> = find_symbols(&input_lines);
    let all_numbers: Vec<(i32, HashSet<(i32, i32)>)> = find_numbers(input_lines);

    let total = all_numbers.iter().fold(0, |acc, (num, positions)| {
        if positions.intersection(&all_symbols).count() > 0 {
            acc + num
        } else {
            acc
        }
    });

    format!("{}", total)
}

fn symbol_x_coordinate(col: i32, row: &str) -> HashSet<(i32, i32)> {
    row.char_indices()
        .filter(|&(_, c)| c.is_ascii_punctuation() && c != '.')
        .map(|(i, _)| (col, i as i32))
        .collect()
}

fn find_symbols(lines: &[String]) -> HashSet<(i32, i32)> {
    let mut symbols: HashSet<(i32, i32)> = HashSet::new();

    for (i, line) in lines.iter().enumerate() {
        let line_symbols: HashSet<(i32, i32)> = symbol_x_coordinate(i as i32, line);
        symbols.extend(&line_symbols);
    }
    symbols
}

fn number_x_coordinates(col: i32, line: &str) -> Vec<(i32, HashSet<(i32, i32)>)> {
    let mut numbers: Vec<(i32, HashSet<(i32, i32)>)> = Vec::new();
    let mut num: Vec<char> = Vec::new();
    let mut num_positions: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..line.len() {
        let current_char = line.chars().nth(i).unwrap();

        if !num.is_empty() && current_char.is_ascii_punctuation() {
            numbers.push((
                String::from_iter(num).parse::<i32>().unwrap(),
                num_positions
                    .iter()
                    .flat_map(|&p| adjacent_positions(p))
                    .collect(),
            ));

            num = Vec::new();
            num_positions = HashSet::new();
        }
        if current_char.is_numeric() {
            num.push(current_char);
            num_positions.insert((col, i as i32));
        }
    }

    if !num.is_empty() {
        numbers.push((
            String::from_iter(num).parse::<i32>().unwrap(),
            num_positions
                .iter()
                .flat_map(|&p| adjacent_positions(p))
                .collect(),
        ));
    }
    numbers
}

fn find_numbers(lines: Vec<String>) -> Vec<(i32, HashSet<(i32, i32)>)> {
    let mut numbers: Vec<(i32, HashSet<(i32, i32)>)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let line_numbers: Vec<(i32, HashSet<(i32, i32)>)> =
            number_x_coordinates(i as i32, line);
        numbers.extend(line_numbers);
    }
    numbers
}

fn adjacent_positions((row, col): (i32, i32)) -> HashSet<(i32, i32)> {
    let mut adjacents: HashSet<(i32, i32)> = HashSet::new();

    for i in -1..2 {
        for j in -1..2 {
            adjacents.insert((row + i, col + j));
        }
    }

    adjacents
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use support::read_input_file_as_lines;

    use super::*;

    #[test]
    fn return_symbol_x_coordinate() {
        let result: HashSet<(i32, i32)> = HashSet::new();
        assert_eq!(result, symbol_x_coordinate(2, "467..114.."));
        assert_eq!(
            HashSet::from([(0, 3)]),
            symbol_x_coordinate(0, "...*......")
        );
        assert_eq!(
            HashSet::from([(1, 6)]),
            symbol_x_coordinate(1, "......#...")
        );
        assert_eq!(
            HashSet::from([(4, 3)]),
            symbol_x_coordinate(4, "617*......")
        );
        assert_eq!(
            HashSet::from([(4, 5)]),
            symbol_x_coordinate(4, ".....+.58.")
        );
        assert_eq!(
            HashSet::from([(5, 3), (5, 5)]),
            symbol_x_coordinate(5, "...$.*....")
        );
    }

    #[test]
    fn find_symbols_should_return_all_symbols() {
        let expected: HashSet<(i32, i32)> =
            HashSet::from([(1, 3), (3, 6), (4, 3), (5, 5), (8, 3), (8, 5)]);

        let lines: Vec<String> = read_input_file_as_lines("resource/day03_small");

        assert_eq!(expected, find_symbols(&lines));
    }

    #[test]
    fn return_find_numbers() {
        assert_eq!(
            vec!((617, HashSet::from([(-1, 0), (0, 2), (0, 0), (1, -1), (0, -1), (-1, 3), (1, 1), (-1, -1), (1, 0), (1, 3), (-1, 1), (0, 1), (0, 3), (-1, 2), (1, 2)]))),
            number_x_coordinates(0, "617*......")
        );
        assert_eq!(
            vec!(
                (467, HashSet::from([(1, -1), (1, 2), (0, 3), (0, 0), (0, 1), (1, 1), (2, 2), (0, 2), (2, -1), (2, 1), (1, 0), (0, -1), (2, 0), (2, 3), (1, 3)])),
                (114, HashSet::from([(0, 6), (2, 6), (0, 7), (0, 5), (1, 7), (1, 8), (2, 7), (1, 5), (1, 6), (1, 4), (0, 8), (2, 5), (2, 4), (2, 8), (0, 4)]))
            ),
            number_x_coordinates(1, "467..114..")
        );
        assert_eq!(
            vec!(
                (35, HashSet::from([(3, 4), (2, 3), (3, 2), (1, 2), (2, 4), (1, 3), (1, 4), (2, 2), (3, 5), (2, 5), (1, 5), (3, 3)])),
                (633, HashSet::from([(1, 9), (3, 7), (3, 9), (1, 7), (2, 9), (2, 7), (3, 6), (2, 10), (3, 10), (1, 10), (1, 6), (2, 6), (1, 8), (2, 8), (3, 8)])),
            ),
            number_x_coordinates(2, "...35..633")
        );
    }

    #[test]
    fn return_adjacent_positions() {
        assert_eq!(
            HashSet::from([
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 0),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1)
            ]),
            adjacent_positions((0, 0))
        );
    }

    #[test]
    fn test_first_answer() {
        //         "467..114.."
        // "...*......"
        // "..35..633."
        // "......#..."
        // "617*......"
        // ".....+.58."
        // "..592....."
        // "......755."
        // "...$.*...."
        // ".664.598.."

        assert_eq!("4361", day03_part1_answer("resource/day03_small"));
    }
}
