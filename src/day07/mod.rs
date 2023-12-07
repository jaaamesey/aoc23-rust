use std::collections::HashMap;

const INPUT: &str = include_str!("./test_input.txt");

// Strength is index + 1
const CARDS_BY_STRENGTH: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Copy, Clone, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Copy, Clone, Debug)]
struct Hand<'a> {
    hand: &'a str,
    hand_as_hex: i32,
    hand_type: HandType,
    bid: i32,
}

pub fn part1() {
    let parsed_hands = INPUT.lines().map(|line| -> (&str, i32) {
        let mut columns = line.split_whitespace();
        return (
            columns.next().unwrap(),
            columns.next().unwrap().parse::<i32>().unwrap(),
        );
    });

    let mut hands = parsed_hands
        .map(|(hand, bid)| {
            let hand_type = {
                let card_counts = into_character_counts(hand);

                let mut hand_numbers = card_counts.values().map(|&v| v).collect::<Vec<_>>();
                hand_numbers.sort();

                match hand_numbers[..] {
                    [5] => HandType::FiveOfAKind,
                    [1, 4] => HandType::FourOfAKind,
                    [2, 3] => HandType::FullHouse,
                    [1, 1, 3] => HandType::ThreeOfAKind,
                    [1, 2, 2] => HandType::TwoPair,
                    [1, 1, 1, 2] => HandType::OnePair,
                    _ => HandType::HighCard,
                }
            };
            return Hand {
                hand,
                hand_as_hex: hand_to_hex(hand),
                hand_type,
                bid,
            };
        })
        .collect::<Vec<_>>();

    hands.sort_by(|&a, &b| {
        let (a_hand_score, b_hand_score) = (a.hand_type as i32, b.hand_type as i32);
        if a_hand_score == b_hand_score {
            return a.hand_as_hex.cmp(&b.hand_as_hex);
        }
        return a_hand_score.cmp(&b_hand_score);
    });

    let output = hands
        .iter()
        .enumerate()
        .map(|(i, &hand)| hand.bid * (i + 1) as i32)
        .fold(0, |acc, curr| acc + curr);

    dbg!(output);
}

pub fn part2() {}

fn hand_to_hex(hand: &str) -> i32 {
    let hex = i32::from_str_radix(
        hand.chars()
            .map(|char| {
                CARDS_BY_STRENGTH
                    .iter()
                    .position(|&card| card == char)
                    .unwrap()
            })
            .fold("".to_string(), |acc: String, curr| {
                let hex_digit = &format!("{:X}", curr).to_string();
                acc + hex_digit
            })
            .as_str(),
        16,
    )
    .unwrap();
    return hex;
}

fn into_character_counts(str: &str) -> HashMap<char, i32> {
    str.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}
