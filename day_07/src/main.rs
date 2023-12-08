use std::cmp::Ordering;

mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: String,
    bid: i32,
}

fn get_hand_type(hand: &str) -> HandType {
    let cards = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
    ];

    let mut hand_type = HandType::HighCard;

    for card in cards {
        if hand.matches(card).count() == 2 {
            match hand_type {
                HandType::ThreeOfAKind => hand_type = HandType::FullHouse,
                HandType::OnePair => hand_type = HandType::TwoPair,
                _ => hand_type = HandType::OnePair,
            }
        }
        if hand.matches(card).count() == 3 {
            match hand_type {
                HandType::OnePair => hand_type = HandType::FullHouse,
                _ => hand_type = HandType::ThreeOfAKind,
            }
        }
        if hand.matches(card).count() == 4 {
            hand_type = HandType::FourOfAKind;
        }
        if hand.matches(card).count() == 5 {
            hand_type = HandType::FiveOfAKind;
        }
    }

    hand_type
}

fn part1(input: &str) -> i32 {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|l| {
            let hand: String = l
                .split_ascii_whitespace()
                .nth(0)
                .unwrap()
                .chars()
                .map(|c| match c {
                    'A' => 'A',
                    'K' => 'B',
                    'Q' => 'C',
                    'J' => 'D',
                    'T' => 'E',
                    '9' => 'F',
                    '8' => 'G',
                    '7' => 'H',
                    '6' => 'I',
                    '5' => 'J',
                    '4' => 'K',
                    '3' => 'L',
                    '2' => 'M',
                    _ => c,
                })
                .collect();
            let bid = l.split_ascii_whitespace().nth(1).unwrap();
            let hand_type = get_hand_type(&hand);

            Hand {
                hand_type,
                cards: hand.to_string(),
                bid: bid.parse::<i32>().unwrap(),
            }
        })
        .collect();

    hands.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            for (card_a, card_b) in a.cards.chars().zip(b.cards.chars()) {
                if card_a == card_b {
                    continue;
                }
                return card_a.cmp(&card_b);
            }
            Ordering::Equal
        } else {
            a.hand_type.cmp(&b.hand_type)
        }
    });

    let total: i32 = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, h)| h.bid * (i as i32 + 1))
        .sum();

    total
}

fn part2(input: &str) -> usize {
    0
}
