use std::{fs::{File},error::Error,io::{BufRead,BufReader}};

fn main() -> Result<(), Box<dyn Error>> {
	let number_vec = BufReader::new(File::open("./Day_1/input.txt")?)
		.lines()
		.map(|i| i.unwrap())
		.map(|i| i.parse::<usize>().expect("not a number"))
		.collect();
	println!("part1 result: {}", part1(&number_vec));
	println!("part2 result: {}", part2(&number_vec));
	Ok(())
}

fn part1(numbers: &Vec<usize>) -> isize {
	let mut result: isize = -1;
	let mut previous = 0;
	for number in numbers {
		if *number > previous {result += 1;} 
		previous = *number;
	}
	result
}

fn part2(numbers: &Vec<usize>) -> isize {
	let mut result = -1;
	let mut previous = 0;
	for number in numbers.windows(3)
		.map(|i| i.iter().fold(0, |sum, i| sum+i)) 
	{
		if number > previous {result += 1;} 
		previous = number;
	}
	result
}