use array2d::Array2D;
use std::cmp::min;
mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

fn is_mirrored(pattern: &Vec<String>, index: usize) -> bool {
    let mut copy1 = pattern.iter().take(index).collect::<Vec<&String>>();
    let copy2 = pattern.iter().skip(index).collect::<Vec<&String>>();
    copy1.reverse();
    let len = *min(&copy1.len(), &copy2.len());

    let mut mirrored_lines = vec![];
    for i in 0..len {
        if copy1[i] == copy2[i] {
            mirrored_lines.push(true);
        } else {
            mirrored_lines.push(false);
        }
    }

    if mirrored_lines.contains(&false) {
        return false;
    }

    true
}

fn part1(input: &str) -> i32 {
    let mut total = 0;

    let patterns: Vec<Vec<String>> = input
        .split("\n\n")
        .map(|p| {
            p.lines()
                .map(|l| l.lines().collect())
                .collect::<Vec<String>>()
        })
        .collect();

    for pattern in patterns {
        // check rows
        let horizontal_reflection: Vec<i32> = pattern
            .clone()
            .iter()
            .enumerate()
            .map(|(li, l)| {
                if li > 0 && pattern.iter().nth(li - 1).unwrap().contains(l) {
                    if is_mirrored(&pattern, li) {
                        return li as i32;
                    }
                }
                0
            })
            .collect::<Vec<i32>>();

        // flip the pattern
        let flipped_pattern: Vec<String> = Array2D::from_rows(
            &pattern
                .iter()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        )
        .unwrap()
        .as_columns()
        .iter()
        .map(|l| l.iter().collect::<String>())
        .collect();

        // check cols
        let vertical_reflection: Vec<i32> = flipped_pattern
            .iter()
            .enumerate()
            .map(|(li, l)| {
                if li > 0 && flipped_pattern.iter().nth(li - 1).unwrap().contains(l) {
                    if is_mirrored(&flipped_pattern, li) {
                        return li as i32;
                    }
                }
                0
            })
            .collect::<Vec<i32>>();

        total += horizontal_reflection.iter().sum::<i32>() * 100;
        total += vertical_reflection.iter().sum::<i32>();
    }

    total
}

fn part2(input: &str) -> i64 {
    0
}
