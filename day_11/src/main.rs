use array2d::Array2D;

mod tests;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input);
    let part2 = part2(input);
    dbg!(part1);
    dbg!(part2);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Space,
    Galaxy,
}

// ...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#.....

// ....#........
// .........#...
// #............
// .............
// .............
// ........#....
// .#...........
// ............#
// .............
// .............
// .........#...
// #....#.......

fn part1(input: &str) -> i32 {
    let mut map: Vec<Vec<Tile>> = vec![];

    input.lines().enumerate().for_each(|(y, line)| {
        map.push(vec![]);
        line.chars().for_each(|c| {
            let tile = match c {
                '.' => Tile::Space,
                '#' => Tile::Galaxy,
                _ => panic!("Unknown tile."),
            };
            map[y].push(tile);
        })
    });

    let mut map = Array2D::from_rows(&map).unwrap();
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    // Find empty rows
    for (y, mut row) in map.rows_iter().enumerate() {
        if !row.find(|&&r| r == Tile::Galaxy).is_some() {
            empty_rows.push(y)
        }
    }

    // Find empty cols
    for (x, mut col) in map.columns_iter().enumerate() {
        if !col.find(|&&r| r == Tile::Galaxy).is_some() {
            empty_cols.push(x);
        }
    }

    // Add empty rows
    let mut map_as_rows = map.as_rows();
    empty_rows.iter().enumerate().for_each(|(i, ri)| {
        let mut tiles = vec![];
        for _ in 0..map.row_len() {
            tiles.push(Tile::Space)
        }
        map_as_rows.insert(*ri + i, tiles);
    });
    map = Array2D::from_rows(&map_as_rows).unwrap();

    // Add empty cols
    let mut map_as_cols = map.as_columns();
    empty_cols.iter().enumerate().for_each(|(i, ci)| {
        let mut tiles = vec![];
        for _ in 0..map.column_len() {
            tiles.push(Tile::Space)
        }
        map_as_cols.insert(*ci + i, tiles);
    });
    map = Array2D::from_columns(&map_as_cols).unwrap();

    // collect all galaxies
    let mut galaxies: Vec<(usize, usize)> = map
        .enumerate_row_major()
        .filter_map(|((x, y), tile)| match tile {
            Tile::Galaxy => Some((x, y)),
            _ => None,
        })
        .collect();

    // add up differences between galaxies (x, y)
    let mut count = 0;
    while galaxies.iter().count() > 1 {
        for i in 0..galaxies.iter().count() {
            let mut dx = galaxies[0].0 as i32 - galaxies[i].0 as i32;
            let mut dy = galaxies[0].1 as i32 - galaxies[i].1 as i32;
            if dx.is_negative() {
                dx = dx * -1;
            }
            if dy.is_negative() {
                dy = dy * -1;
            }
            count += dx + dy;
        }
        galaxies.remove(0);
    }

    count
}

const MULTIPLIER: usize = 999999;

fn part2(input: &str) -> i64 {
    let mut map: Vec<Vec<Tile>> = vec![];

    input.lines().enumerate().for_each(|(y, line)| {
        map.push(vec![]);
        line.chars().for_each(|c| {
            let tile = match c {
                '.' => Tile::Space,
                '#' => Tile::Galaxy,
                _ => panic!("Unknown tile."),
            };
            map[y].push(tile);
        })
    });

    let map = Array2D::from_rows(&map).unwrap();
    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    // Find empty rows
    for (y, mut row) in map.rows_iter().enumerate() {
        if !row.find(|&&r| r == Tile::Galaxy).is_some() {
            empty_rows.push(y)
        }
    }

    // Find empty cols
    for (x, mut col) in map.columns_iter().enumerate() {
        if !col.find(|&&r| r == Tile::Galaxy).is_some() {
            empty_cols.push(x);
        }
    }

    // collect all galaxies, apply multiplier to (x, y)
    let mut galaxies: Vec<(usize, usize)> = map
        .enumerate_row_major()
        .filter_map(|((x, y), tile)| {
            let mut cy = 0;
            for fy in empty_cols.iter() {
                if y > *fy {
                    cy += MULTIPLIER;
                }
            }
            let mut cx = 0;
            for fx in empty_rows.iter() {
                if x > *fx {
                    cx += MULTIPLIER;
                }
            }
            match tile {
                Tile::Galaxy => Some((x + cx, y + cy)),
                _ => None,
            }
        })
        .collect();

    dbg!(&galaxies);

    // add up differences between galaxies (x, y)
    let mut count = 0;
    while galaxies.iter().count() > 1 {
        for i in 0..galaxies.iter().count() {
            let mut dx = galaxies[0].0 as i64 - galaxies[i].0 as i64;
            let mut dy = galaxies[0].1 as i64 - galaxies[i].1 as i64;
            if dx.is_negative() {
                dx = dx * -1;
            }
            if dy.is_negative() {
                dy = dy * -1;
            }
            count += dx + dy;
        }
        galaxies.remove(0);
    }

    count
}
