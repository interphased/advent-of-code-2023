mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

fn part1(input: &str) -> i32 {
    let mut totals: Vec<i32> = vec![];

    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|c| c.parse::<i32>().expect("&str cannot be parsed to i32"))
                .collect::<Vec<i32>>()
        })
        .collect();

    for line in lines {
        let mut has_reached_end = false;
        let mut generated_lines: Vec<Vec<i32>> = vec![line.clone()];

        while !has_reached_end {
            let new_last_line = generated_lines.last().unwrap();
            let mut local_line: Vec<i32> = vec![];

            for i in 0..new_last_line.iter().count() {
                if i < new_last_line.iter().count() - 1 {
                    local_line.push(new_last_line[i + 1] - new_last_line[i]);
                }
            }

            generated_lines.push(local_line);

            if generated_lines.last().unwrap().iter().sum::<i32>() == 0 {
                has_reached_end = true;
            }
        }

        generated_lines.reverse();

        for row_index in 0..generated_lines.iter().count() {
            let clone = generated_lines.clone();

            if row_index < generated_lines.iter().count() - 1 {
                generated_lines[row_index + 1]
                    .push(clone[row_index].last().unwrap() + clone[row_index + 1].last().unwrap());
            }
        }

        totals.push(*generated_lines.last().unwrap().last().unwrap());
    }

    totals.iter().sum()
}

fn part2(input: &str) -> i32 {
    let mut totals: Vec<i32> = vec![];

    let lines: Vec<Vec<i32>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|c| c.parse::<i32>().expect("&str cannot be parsed to i32"))
                .collect::<Vec<i32>>()
        })
        .collect();

    for line in lines {
        let mut has_reached_end = false;
        let mut generated_lines: Vec<Vec<i32>> = vec![line.clone()];

        while !has_reached_end {
            let new_last_line = generated_lines.last().unwrap();
            let mut local_line: Vec<i32> = vec![];

            for i in 0..new_last_line.iter().count() {
                if i < new_last_line.iter().count() - 1 {
                    local_line.push(new_last_line[i + 1] - new_last_line[i]);
                }
            }

            generated_lines.push(local_line);

            if generated_lines.last().unwrap().iter().sum::<i32>() == 0 {
                has_reached_end = true;
            }
        }

        generated_lines.reverse();

        generated_lines.iter_mut().for_each(|l| l.reverse());

        for row_index in 0..generated_lines.iter().count() {
            let clone = generated_lines.clone();

            if row_index < generated_lines.iter().count() - 1 {
                generated_lines[row_index + 1]
                    .push(clone[row_index + 1].last().unwrap() - clone[row_index].last().unwrap());
            }
        }

        totals.push(*generated_lines.last().unwrap().last().unwrap());
    }

    totals.iter().sum()
}
