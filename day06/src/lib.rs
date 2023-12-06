pub fn day06_part1_answer() -> String {

    //Time:        54     81     70     88
    //Distance:   446   1292   1035   1007
    
    let input = vec![(54, 466), (81, 1292), (70, 1035), (88, 1007)];

    let answer: u128 = input.iter().map(
        |&(dur, dist)| record_beaters(dur, dist)
    ).product();

    format!("{}", answer)
}

pub fn day06_part2_answer() -> String {

    // Time:        5_481_088
    // Distance:    446_129_210_351_007
    
    let answer: u128 = record_beaters_big(5_481_088, 446_129_210_351_007);

    format!("{}", answer)
}


fn calc_distance(press: u128, duration: u128) -> f64 {
    let run_distance = (duration as f64) - (press as f64);

    run_distance * press as f64
}

fn record_beaters(duration: u128, record: u128) -> u128 {
    (1..duration).into_iter()
    .filter(|&p| calc_distance(p, duration) > record as f64)
    .fold(0, |acc, _| acc + 1)
}

fn record_beaters_big(duration: u128, record: u128) -> u128 {

    let low_end = (0..duration).into_iter()
    .take_while(|&p| calc_distance(p, duration) <= record as f64)
    .fold(0, |acc, _| acc + 1);

    let high_end = (0..=duration).rev().into_iter()
    .take_while(|&p| calc_distance(p, duration) <= record as f64 )
    .fold(0, |acc, _| acc + 1);

    println!("{}, {}", high_end, low_end);

    duration +1 - low_end - high_end
}

#[cfg(test)]
mod tests {
    use super::*;

    //"Time:      7  15   30"
    //"Distance:  9  40  200"

    #[test]
    fn calc_distance_returns_distance_given_time() {
        
        assert_eq!(0f64, calc_distance(0, 7));
        assert_eq!(6f64, calc_distance(1, 7));
        assert_eq!(10f64, calc_distance(2, 7));
        assert_eq!(12f64, calc_distance(3, 7));
        assert_eq!(12f64, calc_distance(4, 7));
        assert_eq!(10f64, calc_distance(5, 7));
        assert_eq!(6f64, calc_distance(6, 7));
        assert_eq!(0f64, calc_distance(7, 7));
    }

    #[test]
    fn record_beaters_returns_winning_times(){
        assert_eq!(4, record_beaters(7, 9));
        assert_eq!(8, record_beaters(15, 40));
        assert_eq!(9, record_beaters(30, 200));
        assert_eq!(1, record_beaters(5_481_088, 446_129_210_351_007));

    }

    #[test]
    fn record_beaters_big_returns_winning_times(){
        assert_eq!(4, record_beaters_big(7, 9));
        assert_eq!(8, record_beaters_big(15, 40));
        assert_eq!(9, record_beaters_big(30, 200));

        assert_eq!(1, record_beaters_big(5_481_088, 446_129_210_351_007));
    }

}
