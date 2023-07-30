use std::{io::{Write, BufReader, BufWriter}, fs::File};

use crate::models::Customer;

// Macros
macro_rules! read_t{
	($t:tt) => {{
			let mut temp = String::new();
			std::io::stdin().read_line(&mut temp).expect("fail");
			temp.trim().parse::<$t>()
	}};
}

//Constants
const FILE_PATH: &str = "src/database.json";

pub fn get_int_input(upper_bound: u32) -> u32 {
	let res = loop {
		match read_t!(u32) {
				Ok(i) => if i > upper_bound || i == 0 {
						print_prompt("Invalid selection, try again: ")
				} else {
						break i
				},
				_ => {            
					print_prompt("Invalid input, try again: ");
				}
		}
	};

	res
}


pub fn get_string_input() -> String {
	let res = read_t!(String).unwrap();
	res
}

pub fn print_prompt(prompt: &str) {
	print!("{}", prompt);
	std::io::stdout().flush().unwrap();
}

pub fn prompt(prompt: &str) -> String {
	print!("{}", prompt);
	std::io::stdout().flush().unwrap();
	get_string_input()
}


// pub fn get_db_info() -> File {
// 	let database = OpenOptions::new()
// 	.write(true)
// 	.read(true)
// 	.open(FILE_PATH)
// 	.expect("cannot open file");
// 	return database;
// }

pub fn save_customer(customer: Customer) -> Result<bool, String>{
	let mut customers= read_database();

	customers.push(customer);
	overwrite_db(customers);
	Ok(true)
}

pub fn read_database() -> Vec<Customer> {
	let file = File::open(FILE_PATH).unwrap();
	let reader = BufReader::new(file);
	let customers: Vec<Customer> =  match serde_json::from_reader(reader) {
		Ok(i) => i,
		_ => {Vec::new()}
	};

	customers
}

pub fn overwrite_db(info: Vec<Customer>) {	
	let db = File::create(FILE_PATH).unwrap();
	let mut writer = BufWriter::new(db);
	serde_json::to_writer(&mut writer, &info).unwrap();
}

