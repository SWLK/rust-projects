// employees:

// Using a hash map and vectors, create a text interface to allow
// a user to add employee names to a department in a company.
// For example, "Add Sally to Engineering" or "Add Amir to Sales".
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.
// ===================================================================

use std::io;
use std::collections::HashMap;

pub fn main() {
	let mut book: HashMap<String, String> = HashMap::new();
	// interface to allow choice of adding or displaying
	loop {
		println!(
			"=================================\n\
			1. Add an employee\n\
			2. Display employees\n\
			3. Quit\n\
			=================================\n"
		);

		let mut choice = String::new();

		io::stdin()
			.read_line(&mut choice)
			.expect("Failed to read line");

		let choice = match choice.trim().parse::<i8>() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match choice {
			1 => {
				let (key, value) = get_info();
				book.insert(key, value);
			},
			2 => {
				loop {
					println!(
						"==============================\n\
						1. List employees in department\n\
						2. List all employees\n\
						3. Back to menu\n\
						==============================\n"
					);

					let mut choice = String::new();

					io::stdin()
						.read_line(&mut choice)
						.expect("Unable to read line");

					match choice.trim().parse::<i32>() {
						Ok(1) => department_list(&book),
						Ok(2) => full_list(&book),
						_ => break,
					}
				}
			},
			_ => break,
		}
	}
}

fn get_info() -> (String, String) {
	println!("Enter employee name:");

	let mut name = String::new();
	
	io::stdin()
		.read_line(&mut name)
		.expect("Unable to read name");

	let name = String::from(name.trim());

	println!("Enter employee department:");

	let mut department = String::new();

	io::stdin()
		.read_line(&mut department)
		.expect("Unable to read department");

	let department = String::from(department.trim());

	(name, department)
}

fn department_list(book: &HashMap<String, String>) {
	println!("Enter the department:");

	let mut department = String::new();

	io::stdin()
		.read_line(&mut department)
		.expect("Unable to read department");

	department = String::from(department.trim());

	let mut names: Vec<&String> = Vec::new();

	for (key, value) in book {
		if department.eq(value) {
			names.push(key);
		}
	}

	names.sort();

	println!("{:#?}", names);
}

fn full_list(book: &HashMap<String, String>) {
	// loop through HashMap to find all types of values, sorted in a vector
	let mut departments: Vec<String> = Vec::new();


	// create a vector for each department found
	for (_key, value) in book {
		if !departments.contains(value) {
			departments.push(String::from(&value[..]));
		}
	}

	let mut employees: Vec<Vec<String>> = Vec::new();

	for _i in 0..departments.len() {
		employees.push(Vec::new());
	}

	// loop through HashMap again and store each employee in corresponding vector
	for (key, value) in book {
		let mut count: usize = 0;
		for department in &departments {
			if department.eq(value) {
				&mut employees[count].push(String::from(&key[..]));
			}
			count += 1;
		}
	}

	// print department name followed by list of employees
	let mut count: usize = 0;
	for department in &departments {
		println!{"Employees in {}:", department};
		&mut employees[count].sort();

		// for employee in &employees[count] {
		// 	println!("-- {}", employee);
		// }
		println!("{:#?}", &employees[count]);
		count += 1;
	}
}