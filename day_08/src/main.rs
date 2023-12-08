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
        let address = l.chars().take(3).collect::<String>();
        let left = l.chars().skip(7).take(3).collect::<String>();
        let right = l.chars().skip(12).take(3).collect::<String>();

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

fn part2(input: &str) -> i64 {
    let directions: Vec<char> = input.lines().nth(0).unwrap().chars().collect();
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    input.lines().skip(2).for_each(|l| {
        let address = l.chars().take(3).collect::<String>();
        let left = l.chars().skip(7).take(3).collect::<String>();
        let right = l.chars().skip(12).take(3).collect::<String>();

        map.insert(address, (left, right));
    });

    let mut steps_vec: Vec<i64> = vec![];

    let starts: Vec<String> = map
        .iter()
        .filter(|m| m.0.ends_with("A"))
        .map(|m| m.0.to_string())
        .collect();

    let ends: Vec<String> = map
        .iter()
        .filter(|m| m.0.ends_with("Z"))
        .map(|m| m.0.to_string())
        .collect();

    let mut founds: HashMap<String, bool> = HashMap::new();

    for start in starts {
        let mut steps = 0;
        let found = false;
        let mut current = start.clone();

        'outer: while !found {
            for &direction in &directions {
                steps += 1;
                let node = map.get(&current).expect("Start node not found");

                if direction == 'L' {
                    if ends.contains(&node.0) {
                        founds.insert(node.0.clone(), true);
                        break 'outer;
                    }
                    current = node.0.clone();
                }
                if direction == 'R' {
                    if ends.contains(&node.1) {
                        founds.insert(node.0.clone(), true);
                        break 'outer;
                    }
                    current = node.1.clone();
                }
                founds.insert(node.0.clone(), false);
            }
        }

        steps_vec.push(steps);
    }

    let mut total: i64 = steps_vec[0];
    for i in 0..steps_vec.iter().count() {
        total = num::integer::lcm(total, steps_vec[i]);
    }

    total
}
