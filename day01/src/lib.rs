use support::read_input_file_as_lines;
use regex::Regex;

pub fn day01_part1_answer() -> String {
    // let input_lines = read_input_file_as_lines("resource/day01_input");
    let input_lines = read_input_file_as_lines("day01/resource/day01_input");
    let answer = total_inputs(input_lines);

    format!("{}", answer)
}

fn extract_digits(line: &str) -> (i32, i32) {
    let re = Regex::new(r"(?<digit>\d)").unwrap();
    let captures: Vec<i32> = re.captures_iter(line)
        .map(|i| i.name("digit").unwrap()
            .as_str()
            .parse::<i32>().unwrap())
        .collect();

    let first_digit = *captures.first().unwrap();
    let second_digit = *captures.last().unwrap();

    (first_digit, second_digit)
}

fn sum_digits((first_digit, second_digit):(i32, i32)) -> i32 {
    (first_digit * 10) + second_digit
}

fn total_inputs(lines: Vec<String>) -> i32 {
    lines.iter().map(|l| extract_digits(l))
    .map(self::sum_digits)
    .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_extract_digits_from_line(){

        assert_eq!((1,2), extract_digits("1abc2"));
        assert_eq!((3,8), extract_digits("pqr3stu8vwx"));
        assert_eq!((1,5), extract_digits("a1b2c3d4e5f"));
        assert_eq!((7,7), extract_digits("treb7uchet"));

    }

    #[test]
    fn test_sum_digits(){

        assert_eq!(12, sum_digits((1,2)));
        assert_eq!(38, sum_digits((3,8)));
        assert_eq!(15, sum_digits((1,5)));
        assert_eq!(77, sum_digits((7,7)));

    }

    #[test]
    fn test_sum_inputs(){
        
        let inp: Vec<String> = vec!(
            String::from("1abc2"), 
            String::from("pqr3stu8vwx"), 
            String::from("a1b2c3d4e5f"), 
            String::from("treb7uchet")
        );

        assert_eq!(142, total_inputs(inp));

    }
}
