// average:

// Given a list of integers, use a vector and return the mean (average value),
// median (when sorted, the value in the middle position), and mode
// (the value that occurs most often) of the list.
// ===========================================================================
use std::collections::HashMap;

pub fn main() {
	// Creating a list of integers
	let list = vec![6, 3, 4, 0, 2, 1, 6, 2];

	let mean = mean(&list);

	let median = median(&list);

	let mode = mode(&list);

	println!("The mean is: {}", mean);
	println!("The median is: {}", median);
	println!("The mode is: {:?}", mode);

	println!("{:?}", &list);
}

fn mean(v: &Vec<i32>) -> f32 {
	// adding all the values together
	let mut sum = 0;
	for i in v {
		sum += i;
	}

	// get the length of the vector
	let len = v.len() as f32;

	// divide the total by the length
	sum as f32 / len
}

fn median(v: &Vec<i32>) -> i32 {
	// making a new vector to sort
	let mut temp: Vec<i32> = Vec::new();

	// deep copy
	for i in v {
		temp.push(*i);
	}

	// sort
	temp.sort();

	// get length
	let len = temp.len();

	// match temp.get(len / 2usize) {
	// 	Some(median) => median,
	// 	None => 0,
	// }

	temp[len/2usize]
}

fn mode(v: &Vec<i32>) -> Vec<i32> {
	// creating a hashmap to store frequency of values
	let mut frequency: HashMap<&i32, i32> = HashMap::new();

	// record frequency of values
	for i in v {
		let count = frequency.entry(i).or_insert(0);
		*count += 1;
	}

	// making a vector to store the most appeared values
	let mut max = 0;
	let mut mode: Vec<i32> = Vec::new();

	for (key, value) in &frequency {
		if *value > max {
			max = *value;
			mode = Vec::new();
			mode.push(**key);
		} else if *value == max {
			mode.push(**key);
		}	
	}
	
	mode
}