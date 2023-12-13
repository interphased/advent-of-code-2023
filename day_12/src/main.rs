mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

#[derive(Debug)]
struct Record {
    tiles: Vec<Tile>,
    counts: Vec<usize>,
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Operational,
    Damaged,
    Unknown,
}

fn part1(input: &str) -> i32 {
    let records: Vec<Record> = input
        .lines()
        .map(|l| {
            let (tiles, counts) = l.split_once(' ').unwrap();
            let tiles = tiles
                .chars()
                .map(|c| match c {
                    '#' => Tile::Damaged,
                    '.' => Tile::Operational,
                    '?' => Tile::Unknown,
                    _ => panic!("Unknown tile"),
                })
                .collect::<Vec<Tile>>();
            let counts = counts
                .split(",")
                .map(|c| c.parse::<usize>().expect("Not a number"))
                .collect::<Vec<usize>>();
            Record { tiles, counts }
        })
        .collect();

    // let records: Vec<Record> = input
    //     .lines()
    //     .map(|l| {
    //         l.split_ascii_whitespace()
    //             .nth(1)
    //             .unwrap()
    //             .split(",")
    //             .map(|c| c.parse::<i32>().expect("Not a number"))
    //             .collect::<Vec<i32>>()
    //     })
    //     .collect();

    dbg!(&records);

    // for line in tiles.iter() {
    //     let counter = 0;
    //     for tile in line {
    //         if *tile == Tile::Unknown {
    //             // do something
    //         }
    //     }
    // }

    0
}

fn part2(input: &str) -> i64 {
    0
}
