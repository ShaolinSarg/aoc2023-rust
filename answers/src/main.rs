use day01::{day01_part1_answer, day01_part2_answer};
use day03::day03_part1_answer;

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
        "the answer to day 1 part 2 is: {}",
        day03_part1_answer("day03/resource/day03_input")
    );
}
