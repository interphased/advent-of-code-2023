#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_1() {
        let result = part1(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, 374);
    }

    //     #[test]
    //     fn test_2() {
    //         let result = part2(
    //             "...#......
    // .......#..
    // #.........
    // ..........
    // ......#...
    // .#........
    // .........#
    // ..........
    // .......#..
    // #...#.....",
    //         );
    //         assert_eq!(result, 1030);
    //     }

    #[test]
    fn test_3() {
        let result = part2(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, 8410);
    }
}
