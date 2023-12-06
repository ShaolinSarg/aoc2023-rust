use std::fs;

pub fn read_input_file_as_string(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn read_input_file_as_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn read_input_to_typed_groups<T>(path: &str, op: fn(&str) -> T) -> Vec<Vec<T>> {
    read_input_file_as_lines(path)
        .split(String::is_empty)
        .map(Vec::from)
        .map(|i| i.iter().map(|s| op(s)).collect())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_input_file_returns_string() {
        let result: Vec<String> = read_input_file_as_lines("resource/day01_small");
        assert_eq!(
            result,
            vec!["1000", "2000", "3000", "", "4000", "", "5000", "6000"]
        );
    }

    #[test]
    fn read_input_to_typed_groups_returns_string() {
        let result: Vec<Vec<String>> =
            read_input_to_typed_groups("resource/day01_small", |s| s.to_owned());

        assert_eq!(
            result,
            vec![
                vec!["1000", "2000", "3000"],
                vec!["4000"],
                vec!["5000", "6000"]
            ]
        );
    }

    #[test]
    fn read_input_to_typed_groups_returns_i32() {
        let result: Vec<Vec<i32>> =
            read_input_to_typed_groups("resource/day01_small", |s| s.parse::<i32>().unwrap());

        assert_eq!(
            result,
            vec![vec![1000, 2000, 3000], vec![4000], vec![5000, 6000]]
        );
    }

    #[test]
    fn read_input_to_typed_groups_returns_float() {
        let result: Vec<Vec<f32>> =
            read_input_to_typed_groups("resource/day01_small", |s| s.parse::<f32>().unwrap());

        assert_eq!(
            result,
            vec![
                vec![1000.0, 2000.0, 3000.0],
                vec![4000.0],
                vec![5000.0, 6000.0]
            ]
        );
    }
}
