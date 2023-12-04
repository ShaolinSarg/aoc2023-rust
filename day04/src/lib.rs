use regex::Regex;
use support::read_input_file_as_lines;
use std::collections::HashSet;

pub fn day04_part1_answer(path: &str) -> String {
    let input_lines = read_input_file_as_lines(path);

    let answer :i32 = input_lines.iter().map(|l| group_numbers(l)).map(|c| calculate_points(c)).sum();

    format!("{}", answer)
}

fn group_numbers(line: &str) -> (HashSet<i32>, HashSet<i32>) {
    let re = Regex::new(r":\s(?<winning>.*)\s\|\s(?<our_numbers>.*)").unwrap();
    let captures: Vec<(&str, &str)> = re
        .captures_iter(line)
        .map(|i| {
            (
                i.name("winning").unwrap().as_str(),
                i.name("our_numbers").unwrap().as_str(),
            )
        })
        .collect();

    let (winner_line, our_numbers_line) = *captures.first().unwrap();

    let winners: HashSet<i32> = winner_line
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    let our_numbers: HashSet<i32> = our_numbers_line
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    (winners, our_numbers)
}

fn calculate_points((winning, our_numbers):(HashSet<i32>, HashSet<i32>)) -> i32 {
    let our_winners = our_numbers.intersection(&winning).count() as u32;
    let base: i32 = 2;

    if our_winners == 0 {
        0
    } else {
        base.pow(our_winners-1)
    }

}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn group_numbers_should_return_winners_and_our_numbers() {
        let line = "Card 1: 41 48 83 86 17 | 83 86 6  31 17  9 48 53";
        let expected: (HashSet<i32>, HashSet<i32>) = (
            HashSet::from([41, 48, 83, 86, 17]),
            HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
        );

        assert_eq!(expected, group_numbers(line));
    }

    #[test]
    fn calculate_points_should_return_the_points_on_a_card() {
        let card_numbers: (HashSet<i32>, HashSet<i32>) = (
            HashSet::from([41, 48, 83, 86, 17]),
            HashSet::from([83, 86, 6, 31, 17, 9, 48, 53])
        );
        

        assert_eq!(8, calculate_points(card_numbers));
    }

    #[test]
    fn test_first_answer() {
        assert_eq!("13", day04_part1_answer("resource/day04_small"));
    }
    
}
