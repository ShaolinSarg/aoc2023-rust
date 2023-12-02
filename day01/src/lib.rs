use regex::Regex;
use support::read_input_file_as_lines;

pub fn day01_part1_answer(path: &str) -> String {
    let input_lines = read_input_file_as_lines(path);
    let answer = total_inputs(input_lines);

    format!("{}", answer)
}

pub fn day01_part2_answer(path: &str) -> String {
    let input_lines = read_input_file_as_lines(path);
    let processed_lines = input_lines.iter().map(|l| pre_process_line(l)).collect();

    let answer = total_inputs(processed_lines);

    format!("{}", answer)
}

fn extract_digits(line: &str) -> (i32, i32) {
    let re = Regex::new(r"(?<digit>\d)").unwrap();
    let captures: Vec<i32> = re
        .captures_iter(line)
        .map(|i| i.name("digit").unwrap().as_str().parse::<i32>().unwrap())
        .collect();

    let first_digit = *captures.first().unwrap();
    let second_digit = *captures.last().unwrap();

    (first_digit, second_digit)
}

fn sum_digits((first_digit, second_digit): (i32, i32)) -> i32 {
    (first_digit * 10) + second_digit
}

fn total_inputs(lines: Vec<String>) -> i32 {
    lines
        .iter()
        .map(|l| extract_digits(l))
        .map(self::sum_digits)
        .sum()
}

fn pre_process_line(line: &str) -> String {
    let replacements: Vec<(&str, &str)> = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    line.chars().fold(String::new(), |acc, c| {
        let new = format!("{}{}", acc, c);

        replacements
            .iter()
            .fold(new, |replaced_acc, (word, value)| {
                replaced_acc.replace(word, value)
            })
    })
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
    fn test_pre_process_line() {
        assert_eq!("219", pre_process_line("two1nine"));
        assert_eq!("8wo3", pre_process_line("eightwothree"));
        assert_eq!("abc123xyz", pre_process_line("abcone2threexyz"));
        assert_eq!("x2ne34", pre_process_line("xtwone3four"));
        assert_eq!("49872", pre_process_line("4nineeightseven2"));
        assert_eq!("z1ight234", pre_process_line("zoneight234"));
        assert_eq!("7pqrst6teen", pre_process_line("7pqrstsixteen"));
        assert_eq!("8tkbtzjz698", pre_process_line("eighttkbtzjz6nineeight"));
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
        assert_eq!("55130", day01_part1_answer("resource/day01_input"));
    }

    #[test]
    fn test_second_answer() {
        assert_eq!("54978", day01_part2_answer("resource/day01_input"));
    }
}
