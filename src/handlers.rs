use std::time::Duration;

use crate::{utils::{self, print_prompt, save_customer, read_database, prompt, get_int_input, overwrite_db}, models::{Customer, Account}, main};


pub fn new_customer() {
	let name = prompt("Enter your name: ");
	print_prompt("Enter your PIN code: ");
	let pin_code = loop {
		let input = utils::get_string_input();

		if input.len() != 4 {
			print_prompt("4 digits please, try again: ");
			continue;
		}

		if let Err(_) = input.parse::<i32>() {			
			print_prompt("Input valid digits please, try again: ");
			continue;
		}
		break input
	};

	let customer: Customer = Customer { pin_code, name, accounts: Vec::new() };
	if save_customer(customer.clone()).is_ok() {
		println!("Congratulations, you have registered for the RUST bank");
		println!("Here are your details \n {:?}", customer);
	}
}

pub fn deposit_money () {
	let name = prompt("Enter your name: ");
	let pin_code = prompt("Please enter your PIN code for verification: ");

	let customers: Vec<Customer> = read_database();
	let (customer_index, customer) = if let Some(c) = customers.iter().enumerate().find(|&(_, customer)| customer.name == name && customer.pin_code == pin_code) {
		c
	} else {
		println!("Sorry, it seems you have not registered with us. Thank you");
		return
	};

	if customer.accounts.len() > 0 {
		println!("{:?}", customer.accounts);
		let selected_id = prompt("Select the ID of the account you would like to withdraw from: ");
		println!("ID received: {} ", selected_id);
	} else {
		println!("Create an account");
		create_account(customers.to_owned(), customer_index)

		// create_account(customers, customers.iter().position(|x| x. == customer).unwrap())

		//Todo: Write a function that takes ownership of the array of customers as well as the index of this current customer and creates an account in their account array
	}
}

fn create_account(customers: Vec<Customer>, customer_index: usize) {
	let mut customers = customers;
	let account_type = loop {
		match prompt("Select account type(C/S): ").as_str() {
			"C"	=> break String::from("current"),
			"S" => break String::from("savings"),
			_ => println!("Invalid input")
		}
	};
	println!("Pick an account number, between 1 to 9999");
	let account_number= get_int_input(9999);
	let account_number = account_number.to_string();
	let account: Account = Account { account_number: account_number.clone(), account_type, balance: 0 };

	let index = customer_index;
	let existing_accounts = &customers[index];
	let existing_accounts = &existing_accounts.accounts;

	if existing_accounts.len() > 0 {
		for i in 0..existing_accounts.len() {
			if existing_accounts[i].account_number == account_number{
				println!("An account with the given account number already exists");
				return;
			}
		}

	} else {
		customers[customer_index].accounts.push(account);
	}

	overwrite_db(customers);
	println!("\n Account saved! Thank you \n");
	std::thread::sleep(Duration::new(2, 0));
	println!("Redirecting to main menu...\n");
	std::thread::sleep(Duration::new(3, 0));
	main()
}