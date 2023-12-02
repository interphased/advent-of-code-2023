mod tests;

fn main() {
	let input = include_str!("input.txt");
	let part1 = part1(input);
	let part2 = part2(input);
	dbg!(part1);
	dbg!(part2);
}

fn part1(input: &str) -> String {
	let mut total = 0;

	for line in input.lines() {
		let mut first_char = None;
		let mut last_char = None;

		for char in line.chars() {
			if char.is_numeric() {
				first_char = Some(char);
				break;
			}
		}

		for char in line.chars().rev() {
			if char.is_numeric() {
				last_char = Some(char);
				break;
			}
		}

		if first_char.is_some() && last_char.is_some() {
			let amount = format!("{}{}", first_char.unwrap(), last_char.unwrap());
			total += amount.parse::<i32>().unwrap();
		}
	}

	total.to_string()
}

fn part2(input: &str) -> String {
	let mut total = 0;

	for line in input.lines() {
		let mut first_char: Option<(usize, char)> = None;
		let mut last_char: Option<(usize, char)> = None;

		let words = vec![
			"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
		];

		for (word_index, word) in words.iter().enumerate() {
			let forwards = line.find(word);
			match forwards {
				Some(index) => {
					let fc = char::from_digit(word_index as u32, 10).unwrap();

					if first_char.is_none()
						|| (first_char.is_some() && first_char.unwrap().0 > index)
					{
						first_char = Some((index, fc));
					}
				}
				None => (),
			}

			let backwards = line.rfind(word);
			match backwards {
				Some(index) => {
					let lc = char::from_digit(word_index as u32, 10).unwrap();

					if last_char.is_none() || (last_char.is_some() && last_char.unwrap().0 < index)
					{
						last_char = Some((index, lc));
					}
				}
				None => (),
			}
		}

		for (char_index, char) in line.chars().enumerate() {
			if char.is_numeric() {
				match first_char {
					Some(_) => {
						if char_index < first_char.unwrap().0 {
							first_char = Some((char_index, char));
						}
					}
					None => first_char = Some((char_index, char)),
				}

				match last_char {
					Some(_) => {
						if char_index > last_char.unwrap().0 {
							last_char = Some((char_index, char));
						}
					}
					None => last_char = Some((char_index, char)),
				}
			}
		}

		if first_char.is_some() && last_char.is_some() {
			let amount = format!("{}{}", first_char.unwrap().1, last_char.unwrap().1);
			total += amount.parse::<i32>().unwrap();
		}
	}

	total.to_string()
}
