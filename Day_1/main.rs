use std::{fs::{File},error::Error,io::{BufRead,BufReader}};

fn main() -> Result<(), Box<dyn Error>> {
	let numbers: Vec<isize> = BufReader::new(File::open("./Day_1/input.txt")?).lines()
		.map(|i| i.unwrap().parse::<isize>().unwrap()).collect();
	println!("part1 result: {}", numbers.windows(2).map(|i| if i[0] < i[1] {1} else {0} ).sum::<isize>());
	println!("part2 result: {}", numbers.windows(3).map(|i| i.iter().sum()).collect::<Vec<isize>>()
		.windows(2).map(|i| if i[0] < i[1] {1} else {0} ).sum::<isize>());
	Ok(())
}
