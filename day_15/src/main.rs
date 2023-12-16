mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(",").collect()
}

fn hash(mut current: i32, c: char) -> i32 {
    current += c as i32;
    current *= 17;
    current %= 256;

    current
}

fn part1(input: &str) -> i32 {
    let steps = parse_input(input);

    steps
        .iter()
        .map(|step| {
            let mut current = 0;
            for c in step.chars() {
                current = hash(current, c);
            }
            current
        })
        .sum::<i32>()
}

fn part2(input: &str) -> i32 {
    0
}
