use day01::{day01_part1_answer, day01_part2_answer};
use day04::day04_part1_answer;

fn main() {
    println!(
        "the answer to day 1 part 1 is: {}",
        day01_part1_answer("day01/resource/day01_input")
    );
    println!(
        "the answer to day 1 part 2 is: {}",
        day01_part2_answer("day01/resource/day01_input")
    );

    println!(
        "the answer to day 4 part 1 is: {}",
        day04_part1_answer("day04/resource/day04_input")
    );
}
