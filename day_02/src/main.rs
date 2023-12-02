mod tests;

fn main() {
	let input = include_str!("input.txt");
	let part1 = part1(input);
	let part2 = part2(input);
	dbg!(part1);
	dbg!(part2);
}

#[derive(Debug)]
struct Game {
	id: i32,
	possible: bool,
	power: i32,
}

#[derive(Debug)]
struct Round {
	red: i32,
	green: i32,
	blue: i32,
}

fn part1(input: &str) -> i32 {
	let mut total = 0;
	let max_red = 12;
	let max_green = 13;
	let max_blue = 14;

	for line in input.lines() {
		let split: Vec<&str> = line.split(": ").collect();

		let mut game = Game {
			id: split[0].replace("Game ", "").parse::<i32>().unwrap(),
			possible: true,
			power: 0,
		};

		let rounds: Vec<&str> = split[1].split("; ").collect();

		for r in rounds {
			let colours: Vec<&str> = r.split(", ").clone().collect();

			let mut round = Round {
				red: 0,
				green: 0,
				blue: 0,
			};

			for colour in colours {
				if colour.contains("red") {
					round.red = colour.split(" red").next().unwrap().parse::<i32>().unwrap();
				}
				if colour.contains("green") {
					round.green = colour
						.split(" green")
						.next()
						.unwrap()
						.parse::<i32>()
						.unwrap();
				}
				if colour.contains("blue") {
					round.blue = colour
						.split(" blue")
						.next()
						.unwrap()
						.parse::<i32>()
						.unwrap();
				}
			}

			if round.red > max_red || round.blue > max_blue || round.green > max_green {
				game.possible = false;
			}
		}

		if game.possible {
			total += game.id;
		}
	}

	total
}

fn part2(input: &str) -> i32 {
	let mut total = 0;

	for line in input.lines() {
		let split: Vec<&str> = line.split(": ").collect();

		let mut game = Game {
			id: split[0].replace("Game ", "").parse::<i32>().unwrap(),
			possible: true,
			power: 0,
		};

		let mut reds: Vec<i32> = vec![];
		let mut greens: Vec<i32> = vec![];
		let mut blues: Vec<i32> = vec![];

		let rounds_vec: Vec<&str> = split[1].split("; ").collect();

		for r in rounds_vec {
			let colours: Vec<&str> = r.split(", ").clone().collect();

			for colour in colours {
				if colour.contains("red") {
					reds.push(colour.split(" red").next().unwrap().parse::<i32>().unwrap());
				}
				if colour.contains("green") {
					greens.push(
						colour
							.split(" green")
							.next()
							.unwrap()
							.parse::<i32>()
							.unwrap(),
					);
				}
				if colour.contains("blue") {
					blues.push(
						colour
							.split(" blue")
							.next()
							.unwrap()
							.parse::<i32>()
							.unwrap(),
					);
				}
			}
		}

		game.power = reds.into_iter().reduce(i32::max).unwrap()
			* greens.into_iter().reduce(i32::max).unwrap()
			* blues.into_iter().reduce(i32::max).unwrap();

		total += game.power;
	}

	total
}
