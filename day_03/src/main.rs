mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

fn is_adjacent_to_symbol(x: isize, y: isize, grid: &Vec<Vec<char>>) -> bool {
    let mut value = false;
    let max_x = grid[0].len() as isize;
    let max_y = grid.len() as isize;

    let matrix = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for (mx, my) in matrix {
        let new_x = x + mx;
        let new_y = y + my;
        if (new_x >= 0 && new_x < max_x) && (new_y >= 0 && new_y < max_y) {
            let char = grid[new_y as usize][new_x as usize];
            if !char.is_numeric() && char != '.' {
                value = true;
            }
        }
    }

    value
}

fn part1(input: &str) -> i32 {
    let mut total: i32 = 0;
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    for (y, line) in input.lines().enumerate() {
        let mut current_num: Vec<char> = vec![];
        let mut should_be_counted: bool = false;

        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                current_num.push(char);

                if !should_be_counted {
                    should_be_counted = is_adjacent_to_symbol(x as isize, y as isize, &grid);
                }
            } else {
                if should_be_counted {
                    let num: String = current_num.into_iter().collect();
                    total += num.parse::<i32>().unwrap();
                }
                current_num = vec![];
                should_be_counted = false;
            }
        }

        if should_be_counted {
            let num: String = current_num.into_iter().collect();
            total += num.parse::<i32>().unwrap();
        }
    }

    return total;
}

fn part2(input: &str) -> i32 {
    let result = 0;

    result
}
