use std::collections::HashSet;
use support::read_input_file_as_lines;

pub fn day03_part1_answer(path: &str) -> String {
    let input_lines = read_input_file_as_lines(path);
    let all_symbols: HashSet<(i32, i32)> = find_symbols(input_lines);

    format!("{}", "")
}

fn symbol_x_coordinate(col: i32, row: &str) -> HashSet<(i32, i32)> {
    row.char_indices()
        .filter(|&(_, c)| c.is_ascii_punctuation() && c != '.')
        .map(|(i, _)| (col, i as i32))
        .collect()
}

fn find_symbols(lines: Vec<String>) -> HashSet<(i32, i32)> {
    let mut symbols: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..lines.len() {
        let line_symbols: HashSet<(i32, i32)> = symbol_x_coordinate(i as i32, &lines[i]);
        symbols.extend(&line_symbols);
    }
    symbols
}

fn find_numbers(line: &str) -> Vec<(i32, Vec<i32>)> {
    let mut numbers: Vec<(i32, Vec<i32>)> = Vec::new();
    let mut num:Vec<char> = Vec::new();
    let mut num_positions: Vec<i32> = Vec::new();

    for i in 0..line.len() {
        let current_char = line.chars().nth(i).unwrap();

        if !num.is_empty() && current_char.is_ascii_punctuation() {
            numbers.push((String::from_iter(num).parse::<i32>().unwrap(), num_positions));
            
            num = Vec::new();
            num_positions = Vec::new();
        }
        if current_char.is_numeric() {
            num.push(current_char);
            num_positions.push(i as i32);
        }
    }

    if !num.is_empty() {
        numbers.push((String::from_iter(num).parse::<i32>().unwrap(), num_positions));
    }
    numbers
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

        assert_eq!(expected, find_symbols(lines));
    }

    #[test]
    fn return_find_numbers() {
        assert_eq!(vec!((617, vec!(0,1,2))), find_numbers("617*......"));
        assert_eq!(vec!((467, vec!(0,1,2)), (114, vec!(5,6,7))), find_numbers("467..114.."));
        assert_eq!(vec!((35, vec!(3,4)), (633, vec!(7,8,9)), ), find_numbers("...35..633"));
    }
}
