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
}
