#[cfg(test)]
mod tests {
	use crate::{part1, part2};

	#[test]
	fn test_1() {
		let result = part1("1abc2");
		assert_eq!(result, "12");
	}

	#[test]
	fn test_2() {
		let result = part1("pqr3stu8vwx");
		assert_eq!(result, "38");
	}

	#[test]
	fn test_3() {
		let result = part1("a1b2c3d4e5f");
		assert_eq!(result, "15");
	}

	#[test]
	fn test_4() {
		let result = part1("treb7uchet");
		assert_eq!(result, "77");
	}

	#[test]
	fn test_5() {
		let result = part2("two1nine");
		assert_eq!(result, "29");
	}

	#[test]
	fn test_6() {
		let result = part2("eightwothree");
		assert_eq!(result, "83");
	}

	#[test]
	fn test_7() {
		let result = part2("abcone2threexyz");
		assert_eq!(result, "13");
	}

	#[test]
	fn test_8() {
		let result = part2("xtwone3four");
		assert_eq!(result, "24");
	}

	#[test]
	fn test_9() {
		let result = part2("4nineeightseven2");
		assert_eq!(result, "42");
	}

	#[test]
	fn test_10() {
		let result = part2("zoneight234");
		assert_eq!(result, "14");
	}

	#[test]
	fn test_11() {
		let result = part2("7pqrstsixteen");
		assert_eq!(result, "76");
	}
}
