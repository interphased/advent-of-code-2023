#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_1() {
        let result = part1(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, 46);
    }

    #[test]
    fn test_2() {
        let result = part2(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, 51);
    }
}
