use std::collections::BTreeMap;

mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

fn part1(input: &str) -> i32 {
    let mut total: i32 = 0;

    for card in input.lines() {
        let mut score = 0;

        let all_nums = card
            .split(": ")
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .split(" | ")
            .collect::<Vec<_>>();

        let my_nums: Vec<_> = all_nums.first().unwrap().split(" ").collect();

        let winning_nums: Vec<_> = all_nums
            .last()
            .unwrap()
            .split(" ")
            .filter(|n| !n.is_empty())
            .collect();

        for num in my_nums {
            if winning_nums.contains(&num) {
                if score == 0 {
                    score += 1;
                } else {
                    score *= 2
                }
            }
        }

        total += score;
    }

    return total;
}

#[derive(Debug)]
struct Card {
    count: i32,
}

fn part2(input: &str) -> i32 {
    let mut cards: BTreeMap<i32, Card> = BTreeMap::new();

    for (i, card) in input.lines().enumerate() {
        let id: i32 = i as i32 + 1;

        let all_nums = card
            .split(": ")
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .split(" | ")
            .collect::<Vec<_>>();

        let my_nums: Vec<_> = all_nums.first().unwrap().split(" ").collect();

        let winning_nums: Vec<_> = all_nums
            .last()
            .unwrap()
            .split(" ")
            .filter(|n| !n.is_empty())
            .collect();

        let matches = my_nums
            .iter()
            .filter(|num| winning_nums.contains(num))
            .count();

        if let Some(old_card) = cards.insert(id, Card { count: 1 }) {
            let card_entry = cards.get_mut(&id);
            card_entry.unwrap().count += old_card.count;
        };

        let current_card = cards.get(&id).unwrap();

        if matches > 0 {
            for _ in 0..current_card.count {
                for i in (id + 1)..=(matches as i32) + id {
                    if let Some(old_card) = cards.insert(i, Card { count: 1 }) {
                        let card_entry = cards.get_mut(&i).unwrap();
                        card_entry.count += old_card.count;
                    }
                }
            }
        }
    }

    cards.iter().fold(0, |acc, c| acc + c.1.count)
}

// Below: Attempting to do a queue system. Works, but would take all night to process.

// #[derive(Debug, Clone)]
// struct Card {
//     id: i32,
//     // data: String,
//     wins: i32,
// }

// fn get_num_of_wins(card: &str) -> i32 {
//     let mut num_of_wins = 0;

//     let all_nums = card
//         .split(": ")
//         .collect::<Vec<_>>()
//         .last()
//         .unwrap()
//         .split(" | ")
//         .collect::<Vec<_>>();

//     let my_nums: Vec<_> = all_nums.first().unwrap().split(" ").collect();

//     let winning_nums: Vec<_> = all_nums
//         .last()
//         .unwrap()
//         .split(" ")
//         .filter(|n| !n.is_empty())
//         .collect();

//     for num in my_nums {
//         if winning_nums.contains(&num) {
//             num_of_wins += 1;
//         }
//     }

//     num_of_wins
// }

// fn part2(input: &str) -> i32 {
//     let mut total = 0;
//     let total_num_of_cards = input.lines().count() as i32;
//     let mut card_store: Vec<Card> = vec![];
//     let mut card_queue: Vec<Card> = vec![];

//     // seed card queue
//     for i in 0..total_num_of_cards {
//         total += 1;
//         card_store.push(Card {
//             id: i + 1,
//             wins: get_num_of_wins(input.lines().nth(i as usize).unwrap()),
//         });
//         card_queue.push(Card {
//             id: i + 1,
//             wins: get_num_of_wins(input.lines().nth(i as usize).unwrap()),
//         });
//     }

//     // process queue
//     while card_queue.len() > 0 {
//         let card = &card_queue.first().unwrap();
//         let card_id = card.id.clone();
//         let card_wins = card.wins.clone();

//         if card_wins.is_positive() {
//             for i in 0..card_wins {
//                 let new_card_id = card_id + i + 1;
//                 let new_card_wins = card_store
//                     .iter()
//                     .nth(new_card_id as usize - 1)
//                     .unwrap()
//                     .wins;
//                 if new_card_id <= total_num_of_cards {
//                     total += 1;
//                     card_queue.push(Card {
//                         id: new_card_id,
//                         wins: new_card_wins,
//                     });
//                 }
//             }
//         }

//         card_queue.remove(0);

//         println!("queue length: {}", card_queue.len());
//     }

//     total
// }
