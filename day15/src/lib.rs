//Determine the ASCII code for the current character of the string.
 //   Increase the current value by the ASCII code you just determined.
  //  Set the current value to itself multiplied by 17.
   // Set the current value to the remainder of dividing itself by 256.

pub fn day15_part1_answer(path: &str) -> u32 {
    calc_sum(&support::read_input_file_as_string(path))
}

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |acc, c| {
        let ascii_val = c as u32;
        let added_val = acc + ascii_val;
        let multiplied_val: u32 = added_val as u32 * 17;

        multiplied_val % 256
    })
}

fn calc_sum(input: &str) -> u32 {
    input.split(',').map(|w| hash(w)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
        assert_eq!(30, hash("rn=1"));
        assert_eq!(253, hash("cm-"));
    }

    #[test]
    fn calc_answer() {
        assert_eq!(262, calc_sum("gj=3,jclfth=6,rbs=3"));
    }

    #[test]
    fn calc_day01() {
        assert_eq!(2, day15_part1_answer("resource/input"));
    }
}
