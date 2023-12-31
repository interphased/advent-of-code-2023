mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

fn part1(input: &str) -> i32 {
    let nums: Vec<_> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse::<i32>().expect("Not a number!"))
                .collect::<Vec<i32>>()
        })
        .collect();

    let times = nums.first().unwrap();
    let distances = nums.last().unwrap();
    let mut total = 1;

    for (i, time) in times.iter().enumerate() {
        let mut race_wins = 0;

        for hold in 0..distances[i] {
            if hold != 0 && hold * (time - hold) > distances[i] {
                race_wins += 1;
            }
        }

        total = total * race_wins;
    }

    total
}

fn part2(input: &str) -> usize {
    let nums: Vec<Vec<String>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .map(|n| String::from(n))
                .collect::<Vec<String>>()
        })
        .collect();

    let time = String::from_iter(nums.first().unwrap().clone())
        .parse::<i64>()
        .unwrap();
    let record = String::from_iter(nums.last().unwrap().clone())
        .parse::<i64>()
        .unwrap();

    let total = (0..time)
        .filter_map(|hold| {
            let distance = hold * (time - hold);
            (distance > record).then_some(distance)
        })
        .count();

    total
}
