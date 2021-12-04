use std::{fs::{File},error::Error,io::{BufRead,BufReader}};

fn main() -> Result<(), Box<dyn Error>> {
	let values: Vec<(String, isize)> = BufReader::new(File::open("./Day_2/input.txt")?).lines()
		.map(|i| Ok(i?.split(' ').map(|i| i.to_string()).collect()))
		.map(|i: Result<Vec<String>, Box<dyn Error>>| (i.as_ref().unwrap()[0].clone(),i.as_ref().unwrap()[1].clone().parse::<isize>().unwrap())).collect();		
	let part1_value: (isize, isize) = values.clone().iter().fold((0,0), |(h,v), (dir, val)| match dir.as_str() {"forward" => (h + val,v), "down" => (h, v+val), "up" => (h, v-val), _ => (h,v)});
	let part2_value: (isize, isize, isize) = values.clone().iter().fold((0,0,0), |(h,v,a), (dir, val)| match dir.as_str() {"forward" => (h + val, v + (val * a), a), "down" => (h, v, a+val), "up" => (h, v, a-val), _ => (h,v,a)});


	Ok(())
}