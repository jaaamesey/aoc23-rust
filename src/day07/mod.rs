use std::collections::HashMap;

const INPUT: &str = include_str!("./real_input.txt");

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
struct Hand {
    hand_as_hex: i32,
    hand_score: i32,
    bid: i32,
}

pub fn part1() {
    const CARDS_BY_STRENGTH: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

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
                hand_numbers.sort_unstable();

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
                hand_as_hex: hand_to_hex(hand),
                hand_score: hand_type as i32,
                bid,
            };
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by(|&a, &b| {
        if a.hand_score == b.hand_score {
            return a.hand_as_hex.cmp(&b.hand_as_hex);
        }
        return a.hand_score.cmp(&b.hand_score);
    });

    let output = hands
        .iter()
        .enumerate()
        .map(|(i, &hand)| hand.bid * (i + 1) as i32)
        .fold(0, |acc, curr| acc + curr);

    dbg!(output);
}

pub fn part2() {
    const CARDS_BY_STRENGTH: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

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

    let parsed_hands = INPUT.lines().map(|line| -> (&str, i32) {
        let mut columns = line.split_whitespace();
        return (
            columns.next().unwrap(),
            columns.next().unwrap().parse::<i32>().unwrap(),
        );
    });

    let mut hands = parsed_hands
        .map(|(hand, bid)| {
            let mut hand_variants = Vec::<String>::new();
            hand_variants.push(hand.to_string());
            for c in hand.chars() {
                if c != 'J' {
                    continue;
                }
                for replacement_card in CARDS_BY_STRENGTH {
                    hand_variants.push(hand.replace('J', replacement_card.to_string().as_str()));
                }
            }

            let best_hand_type = hand_variants
                .iter()
                .map(|variant| {
                    let card_counts = into_character_counts(variant);

                    let mut hand_numbers = card_counts.values().map(|&v| v).collect::<Vec<_>>();
                    hand_numbers.sort_unstable();

                    (match hand_numbers[..] {
                        [5] => HandType::FiveOfAKind,
                        [1, 4] => HandType::FourOfAKind,
                        [2, 3] => HandType::FullHouse,
                        [1, 1, 3] => HandType::ThreeOfAKind,
                        [1, 2, 2] => HandType::TwoPair,
                        [1, 1, 1, 2] => HandType::OnePair,
                        _ => HandType::HighCard,
                    }) as i32
                })
                .max()
                .unwrap();

            return Hand {
                hand_as_hex: hand_to_hex(hand),
                hand_score: best_hand_type,
                bid,
            };
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by(|&a, &b| {
        if a.hand_score == b.hand_score {
            return a.hand_as_hex.cmp(&b.hand_as_hex);
        }
        return a.hand_score.cmp(&b.hand_score);
    });

    let output = hands
        .iter()
        .enumerate()
        .map(|(i, &hand)| hand.bid * (i + 1) as i32)
        .fold(0, |acc, curr| acc + curr);

    dbg!(output);
}

fn into_character_counts(str: &str) -> HashMap<char, i32> {
    str.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}
