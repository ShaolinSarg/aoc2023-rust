use day01::{day01_part1_answer, day01_part2_answer};
use day02::day02_part1_answer;
use day03::day03_part1_answer;
use day04::day04_part1_answer;
use day06::{day06_part1_answer, day06_part2_answer};

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
        "the answer to day 2 part 1 is: {}",
        day02_part1_answer("day02/resource/day02_input")
    );

    println!(
        "the answer to day 3 part 1 is: {}",
        day03_part1_answer("day03/resource/day03_input")
    );
    
    println!(
        "the answer to day 4 part 1 is: {}",
        day04_part1_answer("day04/resource/day04_input")
    );

    println!(
        "the answer to day 6 part 1 is: {}",
        day06_part1_answer()
    );

    println!(
        "the answer to day 6 part 2 is: {}",
        day06_part2_answer()
    );

}
