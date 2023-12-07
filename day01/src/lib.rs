mod parser;

use regex::Regex;
use support::read_input_file_as_lines;

pub fn day01_part1_answer(path: &str) -> String {
    let input_lines = read_input_file_as_lines(path);

    let answer = total_inputs(input_lines);

    format!("{}", answer)
}

pub fn day01_part2_answer(path: &str) -> String {
    let input_lines = read_input_file_as_lines(path);
    let parsed_lines: Vec<String> = input_lines
        .iter()
        .map(|l| parser::parse_line(l).unwrap().1)
        .collect();

    let answer = total_inputs(parsed_lines);

    format!("{}", answer)
}

fn extract_digits(line: &str) -> (u64, u64) {
    let re = Regex::new(r"(?<digit>\d)").unwrap();
    let captures: Vec<u64> = re
        .captures_iter(line)
        .map(|i| i.name("digit").unwrap().as_str().parse::<u64>().unwrap())
        .collect();

    let first_digit = *captures.first().unwrap();
    let second_digit = *captures.last().unwrap();

    (first_digit, second_digit)
}

fn sum_digits((first_digit, second_digit): (u64, u64)) -> u64 {
    (first_digit * 10) + second_digit
}

fn total_inputs(lines: Vec<String>) -> u64 {
    lines
        .iter()
        .map(|l| extract_digits(l))
        .map(self::sum_digits)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_extract_digits_from_line() {
        assert_eq!((1, 2), extract_digits("1abc2"));
        assert_eq!((3, 8), extract_digits("pqr3stu8vwx"));
        assert_eq!((1, 5), extract_digits("a1b2c3d4e5f"));
        assert_eq!((7, 7), extract_digits("treb7uchet"));
    }

    #[test]
    fn test_sum_digits() {
        assert_eq!(12, sum_digits((1, 2)));
        assert_eq!(38, sum_digits((3, 8)));
        assert_eq!(15, sum_digits((1, 5)));
        assert_eq!(77, sum_digits((7, 7)));
    }

    #[test]
    fn test_sum_inputs() {
        let inp: Vec<String> = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];

        assert_eq!(142, total_inputs(inp));
    }

    #[test]
    fn test_second_part_totals() {
        let inp: Vec<String> = vec![
            String::from("219"),
            String::from("8wo3"),
            String::from("abc123xyz"),
            String::from("x2ne34"),
            String::from("49872"),
            String::from("z1ight234"),
            String::from("7pqrst6teen"),
        ];

        assert_eq!(281, total_inputs(inp));
    }

    #[test]
    fn test_first_answer() {
        assert_eq!("55130", day01_part1_answer("resource/input"));
    }

    #[test]
    fn test_second_answer() {
        assert_eq!("54978", day01_part2_answer("resource/input"));
    }
}
