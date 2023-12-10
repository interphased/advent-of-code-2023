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
}
