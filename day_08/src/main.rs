use std::collections::HashMap;

mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

fn part1(input: &str) -> i32 {
    let directions: Vec<char> = input.lines().nth(0).unwrap().chars().collect();
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    input.lines().skip(2).for_each(|l| {
        let address = l.split_ascii_whitespace().nth(0).unwrap().to_string();
        let left = l
            .split_ascii_whitespace()
            .nth(2)
            .unwrap()
            .chars()
            .skip(1)
            .take(3)
            .collect::<String>();
        let mut right = l.split_ascii_whitespace().nth(3).unwrap().to_string();
        right.pop();

        map.insert(address, (left, right));
    });

    let mut steps = 0;
    let mut found = false;
    let mut current = "AAA".to_string();
    let end = "ZZZ".to_string();

    while !found {
        for &direction in &directions {
            steps += 1;
            let node = map.get(&current).expect("Start node not found");

            if direction == 'L' {
                if (node.0) == end {
                    found = true;
                }
                current = node.0.clone();
            }
            if direction == 'R' {
                if (node.1) == end {
                    found = true;
                }
                current = node.1.clone();
            }
        }
    }

    steps
}

fn part2(input: &str) -> usize {
    0
}
