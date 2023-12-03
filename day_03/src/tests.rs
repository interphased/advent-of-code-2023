#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_1() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361);
    }
}
