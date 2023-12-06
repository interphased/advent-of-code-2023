mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

#[derive(Debug)]
struct Map {
    destination: i64,
    source: i64,
    end: i64,
}

fn part1(input: &str) -> i64 {
    let mut groups = input.split("\n\n");

    let mut locations: Vec<i64> = groups
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    for group in groups {
        let maps: Vec<Map> = group
            .lines()
            .skip(1)
            .map(|line| {
                let nums: Vec<i64> = line
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect();
                Map {
                    destination: *nums.iter().nth(0).unwrap(),
                    source: *nums.iter().nth(1).unwrap(),
                    end: *nums.iter().nth(1).unwrap() + *nums.iter().nth(2).unwrap(),
                }
            })
            .collect();

        for location in &mut locations {
            for map in &maps {
                if *location >= map.source && *location < map.end {
                    *location = *location - map.source + map.destination;
                    break;
                }
            }
        }
    }

    locations.into_iter().min().unwrap()
}

fn part2(input: &str) -> i64 {
    0
}
