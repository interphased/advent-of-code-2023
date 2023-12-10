use std::collections::HashMap;

mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone)]
enum Tile {
    Start,
    Ground,
    Horizontal,
    Vertical,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

fn part1(input: &str) -> i32 {
    let mut map = HashMap::new();
    let mut starting_location = (Coord { x: 0, y: 0 }, Tile::Start);

    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            let tile = match c {
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                'L' => Tile::TopRight,
                'J' => Tile::TopLeft,
                '7' => Tile::BottomLeft,
                'F' => Tile::BottomRight,
                'S' => {
                    starting_location = (
                        Coord {
                            x: x as isize,
                            y: y as isize,
                        },
                        Tile::Start,
                    );
                    Tile::Start
                }
                _ => Tile::Ground,
            };
            map.insert(
                Coord {
                    x: x as isize,
                    y: y as isize,
                },
                tile,
            );
        })
    });

    let mut steps = 0;
    let mut prev_coords: Coord = Coord {
        x: starting_location.0.x,
        y: starting_location.0.y,
    };
    let mut current_coords: Coord = Coord {
        x: starting_location.0.x + 1,
        y: starting_location.0.y,
    };
    let mut current_tile = map.get(&current_coords);

    'outer: loop {
        steps += 1;
        let mut next_tile = None;
        let mut next_coords = None;

        match current_tile {
            Some(tile) => {
                let nt = match tile {
                    Tile::Horizontal => {
                        let mut next_x = 1;
                        if prev_coords.x > current_coords.x {
                            next_x = -1;
                        }
                        next_coords = Some(Coord {
                            x: current_coords.x + next_x,
                            y: current_coords.y,
                        });
                        map.get(&next_coords.unwrap())
                    }
                    Tile::Vertical => {
                        let mut next_y = 1;
                        if prev_coords.y > current_coords.y {
                            next_y = -1;
                        }
                        next_coords = Some(Coord {
                            x: current_coords.x,
                            y: current_coords.y + next_y,
                        });
                        map.get(&next_coords.unwrap())
                    }
                    Tile::TopLeft => {
                        let mut next_x = 0;
                        let mut next_y = 0;
                        if prev_coords.x < current_coords.x {
                            next_y = -1;
                        }
                        if prev_coords.y < current_coords.y {
                            next_x = -1;
                        }
                        next_coords = Some(Coord {
                            x: current_coords.x + next_x,
                            y: current_coords.y + next_y,
                        });
                        map.get(&next_coords.unwrap())
                    }
                    Tile::TopRight => {
                        let mut next_x = 0;
                        let mut next_y = 0;
                        if prev_coords.x > current_coords.x {
                            next_y = -1;
                        }
                        if prev_coords.y < current_coords.y {
                            next_x = 1;
                        }
                        next_coords = Some(Coord {
                            x: current_coords.x + next_x,
                            y: current_coords.y + next_y,
                        });
                        map.get(&next_coords.unwrap())
                    }
                    Tile::BottomLeft => {
                        let mut next_x = 0;
                        let mut next_y = 0;
                        if prev_coords.x < current_coords.x {
                            next_y = 1;
                        }
                        if prev_coords.y > current_coords.y {
                            next_x = -1;
                        }
                        next_coords = Some(Coord {
                            x: current_coords.x + next_x,
                            y: current_coords.y + next_y,
                        });
                        map.get(&next_coords.unwrap())
                    }
                    Tile::BottomRight => {
                        let mut next_x = 0;
                        let mut next_y = 0;
                        if prev_coords.x > current_coords.x {
                            next_y = 1;
                        }
                        if prev_coords.y > current_coords.y {
                            next_x = 1;
                        }
                        next_coords = Some(Coord {
                            x: current_coords.x + next_x,
                            y: current_coords.y + next_y,
                        });
                        map.get(&next_coords.unwrap())
                    }
                    Tile::Start => break 'outer,
                    _ => panic!("Ground???"),
                };
                next_tile = nt;
            }
            None => panic!("No next location"),
        }

        prev_coords = current_coords.clone();
        current_coords = next_coords.unwrap();
        current_tile = next_tile;
    }

    steps / 2
}

fn part2(input: &str) -> i32 {
    0
}
