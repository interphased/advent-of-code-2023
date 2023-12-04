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

fn part2(input: &str) -> i32 {
    let result = 0;

    result
}
