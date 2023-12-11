#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_1_1() {
        let result = part1(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn test_1_2() {
        let result = part1(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, 8);
    }

    #[test]
    fn test_2_1() {
        let result = part2(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn test_2_2() {
        let result = part2(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );
        assert_eq!(result, 8);
    }

    //     #[test]
    //     fn test_2_3() {
    //         let result = part2(
    //             "FF7FSF7F7F7F7F7F---7
    // L|LJ||||||||||||F--J
    // FL-7LJLJ||||||LJL-77
    // F--JF--7||LJLJ7F7FJ-
    // L---JF-JLJ.||-FJLJJ7
    // |F|F-JF---7F7-L7L|7|
    // |FFJF7L7F-JF7|JL---7
    // 7-L-JL7||F7|L7F-7F7|
    // L.L7LFJ|||||FJL7||LJ
    // L7JLJL-JLJLJL--JLJ.L",
    //         );
    //         assert_eq!(result, 10);
    //     }
}
