use std::collections::{HashMap, HashSet};

mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

#[derive(Debug, Clone, Copy)]
enum TileType {
    Empty,
    ForwardMirror,
    BackMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

#[derive(Debug)]
struct Laser {
    direction: Direction,
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct VisitedTile {
    direction: Direction,
    x: i32,
    y: i32,
}

impl Laser {
    fn travel(&mut self, max_x: usize, max_y: usize, visited: &mut HashSet<VisitedTile>) {
        if visited.contains(&VisitedTile {
            x: self.x,
            y: self.y,
            direction: self.direction,
        }) {
            self.stop();
        } else {
            visited.insert(VisitedTile {
                x: self.x,
                y: self.y,
                direction: self.direction,
            });
        }

        match self.direction {
            Direction::Up => {
                if self.y > 0 {
                    self.y -= 1;
                } else {
                    self.stop();
                }
            }
            Direction::Down => {
                if self.y < max_y as i32 - 1 {
                    self.y += 1;
                } else {
                    self.stop();
                }
            }
            Direction::Right => {
                if self.x < max_x as i32 - 1 {
                    self.x += 1;
                } else {
                    self.stop();
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    self.x -= 1;
                } else {
                    self.stop();
                }
            }
            Direction::Stop => (),
        }
    }

    fn stop(&mut self) {
        self.direction = Direction::Stop;
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    tile_type: TileType,
    energized: bool,
}

fn parse_input(input: &str) -> HashMap<(i32, i32), Tile> {
    let mut map = HashMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let tile_type = match c {
                '.' => TileType::Empty,
                '/' => TileType::ForwardMirror,
                '\\' => TileType::BackMirror,
                '-' => TileType::HorizontalSplitter,
                '|' => TileType::VerticalSplitter,
                _ => panic!("Unknown tile type"),
            };
            map.insert(
                (x as i32, y as i32),
                Tile {
                    tile_type,
                    energized: false,
                },
            );
        })
    });

    map
}

fn part1(input: &str) -> usize {
    let mut map = parse_input(input);
    let max_y = input.lines().count();
    let max_x = input.lines().nth(0).unwrap().chars().count();

    let mut visited_tiles = HashSet::new();

    let mut lasers = vec![Laser {
        direction: Direction::Right,
        x: 0,
        y: 0,
    }];

    let mut lasers_are_still_moving = true;

    while lasers_are_still_moving {
        let mut new_lasers: Vec<Laser> = vec![];
        let mut remove_lasers: Vec<usize> = vec![];

        lasers.iter_mut().enumerate().for_each(|(li, laser)| {
            if laser.direction == Direction::Stop {
                remove_lasers.push(li);
                return;
            }

            let tile = map.get_mut(&(laser.x, laser.y));

            match tile {
                Some(t) => {
                    t.energized = true;
                    match t.tile_type {
                        TileType::Empty => (),
                        TileType::ForwardMirror => match laser.direction {
                            Direction::Up => laser.direction = Direction::Right,
                            Direction::Down => laser.direction = Direction::Left,
                            Direction::Right => laser.direction = Direction::Up,
                            Direction::Left => laser.direction = Direction::Down,
                            _ => (),
                        },
                        TileType::BackMirror => match laser.direction {
                            Direction::Up => laser.direction = Direction::Left,
                            Direction::Down => laser.direction = Direction::Right,
                            Direction::Right => laser.direction = Direction::Down,
                            Direction::Left => laser.direction = Direction::Up,
                            _ => (),
                        },
                        TileType::HorizontalSplitter => match laser.direction {
                            Direction::Up | Direction::Down => {
                                visited_tiles.insert(VisitedTile {
                                    x: laser.x,
                                    y: laser.y,
                                    direction: laser.direction,
                                });
                                laser.direction = Direction::Right;
                                new_lasers.push(Laser {
                                    direction: Direction::Left,
                                    x: laser.x.clone(),
                                    y: laser.y.clone(),
                                });
                            }
                            _ => (),
                        },
                        TileType::VerticalSplitter => match laser.direction {
                            Direction::Left | Direction::Right => {
                                visited_tiles.insert(VisitedTile {
                                    x: laser.x,
                                    y: laser.y,
                                    direction: laser.direction,
                                });
                                laser.direction = Direction::Up;
                                new_lasers.push(Laser {
                                    direction: Direction::Down,
                                    x: laser.x.clone(),
                                    y: laser.y.clone(),
                                });
                            }
                            _ => (),
                        },
                    };
                }
                None => {
                    laser.stop();
                }
            };

            laser.travel(max_x, max_y, &mut visited_tiles);
        });

        lasers.append(&mut new_lasers);

        remove_lasers.iter().enumerate().for_each(|(i, l)| {
            lasers.remove(*l - i);
        });

        if lasers
            .iter()
            .filter(|laser| laser.direction != Direction::Stop)
            .count()
            == 0
        {
            lasers_are_still_moving = false;
        }
    }

    map.iter()
        .filter(|(_, tile)| tile.energized == true)
        .count()
}

fn part2(input: &str) -> i32 {
    0
}
