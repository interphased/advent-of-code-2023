use std::{io::Empty, vec};

mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Round,
    Square,
    Empty,
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Tile::Round,
                    '#' => Tile::Square,
                    '.' => Tile::Empty,
                    _ => panic!("Unknown tile!"),
                })
                .collect::<Vec<Tile>>()
        })
        .collect()
}

fn slide_north(tiles: &mut Vec<Vec<Tile>>) {
    for col in 0..tiles[0].len() {
        let mut destination_row = 0;
        for row in 0..tiles.len() {
            let tile = tiles[row][col];
            match tile {
                Tile::Square => destination_row = row + 1,
                Tile::Round => {
                    let replace_with = std::mem::replace(&mut tiles[destination_row][col], tile);
                    let _ = std::mem::replace(&mut tiles[row][col], replace_with);
                    destination_row += 1;
                }
                Tile::Empty => (),
            }
        }
    }
}

fn calculate_load(tiles: Vec<Vec<Tile>>) -> i32 {
    let mut load = 0;
    for (i, row) in tiles.iter().enumerate() {
        for tile in row {
            load += match tile {
                Tile::Empty => 0,
                Tile::Square => 0,
                Tile::Round => tiles.len() - i,
            }
        }
    }
    load as i32
}

fn part1(input: &str) -> i32 {
    let mut tiles: Vec<Vec<Tile>> = parse_input(input);
    slide_north(&mut tiles);
    calculate_load(tiles)
}

fn part2(input: &str) -> i64 {
    0
}
