use std::{cmp::Ordering, collections::HashMap};

#[derive(PartialEq, Eq, Debug)]
struct Game {
    pub hand: Hand,
    pub bid: u32,
}

impl Game {
    fn new(hand: Hand, bid: u32) -> Self {
        Self { hand, bid }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}
impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Card {
    A,
    K,
    Q,
    J,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn new(input: char) -> Self {
        match input {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Unknown card"),
        }
    }

    fn value(&self) -> u8 {
        match *self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Hand {
    FiveOfAKind(Vec<Card>),
    FourOfAKind(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    TwoPair(Vec<Card>),
    OnePair(Vec<Card>),
    HighCard(Vec<Card>),
}

impl Hand {
    fn new(input: &str) -> Self {
        let cards: Vec<Card> = input.chars().map(Card::new).collect();

        let freqs = cards.iter().fold(HashMap::new(), |mut map, key| {
            map.entry(key).and_modify(|val| *val += 1).or_insert(1);

            map
        });

        match freqs.len() {
            1 => Hand::FiveOfAKind(cards),
            2 if freqs.values().max() == Some(&4) => Hand::FourOfAKind(cards),
            2 => Hand::FullHouse(cards),
            3 if freqs.values().max() == Some(&3) => Hand::ThreeOfAKind(cards),
            3 => Hand::TwoPair(cards),
            4 => Hand::OnePair(cards),
            _ => Hand::HighCard(cards),
        }
    }

    fn strength(&self) -> u8 {
        match *self {
            Hand::FiveOfAKind(_) => 7,
            Hand::FourOfAKind(_) => 6,
            Hand::FullHouse(_) => 5,
            Hand::ThreeOfAKind(_) => 4,
            Hand::TwoPair(_) => 3,
            Hand::OnePair(_) => 2,
            Hand::HighCard(_) => 1,
        }
    }

    fn nth(&self, num: usize) -> &Card {
        match self {
            Hand::FiveOfAKind(cards) => &cards[num],
            Hand::FourOfAKind(cards) => &cards[num],
            Hand::FullHouse(cards) => &cards[num],
            Hand::ThreeOfAKind(cards) => &cards[num],
            Hand::TwoPair(cards) => &cards[num],
            Hand::OnePair(cards) => &cards[num],
            Hand::HighCard(cards) => &cards[num],
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_strength_compare = self.strength().partial_cmp(&other.strength());

        if hand_strength_compare == Some(Ordering::Equal) {
            let first_card_compare = self.nth(0).partial_cmp(other.nth(0));

            if first_card_compare == Some(Ordering::Equal) {
                let second_card_compare = self.nth(1).partial_cmp(other.nth(1));

                if second_card_compare == Some(Ordering::Equal) {
                    let third_card_compare = self.nth(2).partial_cmp(other.nth(2));

                    if third_card_compare == Some(Ordering::Equal) {
                        let fourth_card_compare = self.nth(3).partial_cmp(other.nth(3));

                        if fourth_card_compare == Some(Ordering::Equal) {
                            self.nth(4).partial_cmp(other.nth(4))
                        } else {
                            fourth_card_compare
                        }
                    } else {
                        third_card_compare
                    }
                } else {
                    second_card_compare
                }
            } else {
                first_card_compare
            }
        } else {
            hand_strength_compare
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_strengths = self.strength().cmp(&other.strength());

        if hand_strengths == Ordering::Equal {
            let first_card_compare = self.nth(0).cmp(other.nth(0));

            if first_card_compare == Ordering::Equal {
                let second_card_compare = self.nth(1).cmp(other.nth(1));

                if second_card_compare == Ordering::Equal {
                    let third_card_compare = self.nth(2).cmp(other.nth(2));

                    if third_card_compare == Ordering::Equal {
                        let forth_card_compare = self.nth(3).cmp(other.nth(3));

                        if forth_card_compare == Ordering::Equal {
                            self.nth(4).cmp(other.nth(4))
                        } else {
                            forth_card_compare
                        }
                    } else {
                        third_card_compare
                    }
                } else {
                    second_card_compare
                }
            } else {
                first_card_compare
            }
        } else {
            hand_strengths
        }
    }
}

pub fn day07_part1_answer(path: &str) -> String {
    let input = support::read_input_file_as_lines(path);

    let answer = calculate_winnings(input);

    format!("{}", answer)
}

fn order_games(mut games: Vec<Game>) -> Vec<(usize, Game)> {
    games.sort();

    (1..=games.len()).zip(games.into_iter()).collect()
}

fn calculate_winnings(input: Vec<String>) -> u32 {
    let games = input
        .iter()
        .map(|l| Game::new(Hand::new(&l[0..5]), l[6..].parse::<u32>().unwrap()))
        .collect();

    order_games(games)
        .iter()
        .map(|(rank, game)| *rank as u32 * game.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_new_should_create_strongest_hand() {
        let five_of_a_kind: Hand = Hand::FiveOfAKind(vec![
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::Seven,
        ]);
        assert_eq!(five_of_a_kind, Hand::new("77777"));

        let four_of_a_kind: Hand = Hand::FourOfAKind(vec![
            Card::Seven,
            Card::K,
            Card::Seven,
            Card::Seven,
            Card::Seven,
        ]);
        assert_eq!(four_of_a_kind, Hand::new("7K777"));

        let full_house: Hand = Hand::FullHouse(vec![
            Card::K,
            Card::K,
            Card::Seven,
            Card::Seven,
            Card::Seven,
        ]);
        assert_eq!(full_house, Hand::new("KK777"));

        let three_of_a_kind: Hand = Hand::ThreeOfAKind(vec![
            Card::K,
            Card::A,
            Card::Seven,
            Card::Seven,
            Card::Seven,
        ]);
        assert_eq!(three_of_a_kind, Hand::new("KA777"));

        let two_pair: Hand =
            Hand::TwoPair(vec![Card::K, Card::A, Card::K, Card::Seven, Card::Seven]);
        assert_eq!(two_pair, Hand::new("KAK77"));

        let one_pair: Hand =
            Hand::OnePair(vec![Card::K, Card::A, Card::K, Card::Seven, Card::Five]);
        assert_eq!(one_pair, Hand::new("KAK75"));

        let high_card: Hand = Hand::HighCard(vec![
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten,
            Card::A,
        ]);
        assert_eq!(high_card, Hand::new("789TA"));
    }

    #[test]
    fn compare_should_work_for_uncontested_stronger_hands() {
        let five_of_a_kind = Hand::FiveOfAKind(vec![Card::A, Card::A, Card::A, Card::A, Card::A]);
        let four_of_a_kind = Hand::FourOfAKind(vec![Card::A, Card::A, Card::A, Card::A, Card::K]);
        let full_house = Hand::FullHouse(vec![Card::A, Card::A, Card::A, Card::K, Card::Nine]);
        let three_of_a_kind =
            Hand::ThreeOfAKind(vec![Card::A, Card::A, Card::A, Card::K, Card::Nine]);
        let two_pair = Hand::TwoPair(vec![Card::A, Card::A, Card::K, Card::K, Card::Nine]);
        let one_pair = Hand::OnePair(vec![Card::A, Card::A, Card::K, Card::Q, Card::Nine]);
        let high_card = Hand::HighCard(vec![Card::A, Card::A, Card::K, Card::Q, Card::Nine]);

        assert!(five_of_a_kind > four_of_a_kind);
        assert!(four_of_a_kind > full_house);
        assert!(full_house > three_of_a_kind);
        assert!(three_of_a_kind > two_pair);
        assert!(two_pair > one_pair);
        assert!(one_pair > high_card);
    }

    #[test]
    fn compare_should_work_for_equal_stronger_hands() {
        let full_house_second_high_card = Hand::FullHouse(vec![
            Card::Seven,
            Card::Eight,
            Card::Seven,
            Card::Eight,
            Card::Eight,
        ]);
        let full_house_second_low_card = Hand::FullHouse(vec![
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::Eight,
            Card::Eight,
        ]);

        assert!(full_house_second_high_card > full_house_second_low_card);

        let full_house_third_high_card = Hand::FullHouse(vec![
            Card::Seven,
            Card::Seven,
            Card::Eight,
            Card::Eight,
            Card::Eight,
        ]);
        let full_house_third_low_card = Hand::FullHouse(vec![
            Card::Seven,
            Card::Seven,
            Card::Seven,
            Card::Eight,
            Card::Eight,
        ]);

        assert!(full_house_third_high_card > full_house_third_low_card);

        let two_pair_high_fourth_card =
            Hand::TwoPair(vec![Card::A, Card::A, Card::K, Card::K, Card::Nine]);
        let two_pair_low_fourth_card =
            Hand::TwoPair(vec![Card::A, Card::A, Card::K, Card::Nine, Card::K]);

        assert!(two_pair_high_fourth_card > two_pair_low_fourth_card);

        let one_pair_high_fifth_card =
            Hand::OnePair(vec![Card::A, Card::A, Card::K, Card::Q, Card::Nine]);
        let one_pair_low_fifth_card =
            Hand::OnePair(vec![Card::A, Card::A, Card::K, Card::Q, Card::Eight]);

        assert!(one_pair_high_fifth_card > one_pair_low_fifth_card);
    }

    #[test]
    fn order_games_should_return_hand_orders() {
        let games_ordered: Vec<(usize, Game)> = vec![
            (1, Game::new(Hand::new("32T3K"), 765)),
            (2, Game::new(Hand::new("KTJJT"), 220)),
            (3, Game::new(Hand::new("KK677"), 28)),
            (4, Game::new(Hand::new("T55J5"), 684)),
            (5, Game::new(Hand::new("QQQJA"), 483)),
        ];

        let games_unordered: Vec<Game> = vec![
            Game::new(Hand::new("QQQJA"), 483),
            Game::new(Hand::new("KK677"), 28),
            Game::new(Hand::new("32T3K"), 765),
            Game::new(Hand::new("T55J5"), 684),
            Game::new(Hand::new("KTJJT"), 220),
        ];

        assert_eq!(games_ordered, order_games(games_unordered));
    }
    #[test]
    fn test_small_answer_one() {
        let input = vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ];

        assert_eq!(6440, calculate_winnings(input));
    }

    #[test]
    fn test_answer_one() {
        let input = support::read_input_file_as_lines("resource/input");

        assert_eq!(250232501, calculate_winnings(input));
    }
}
