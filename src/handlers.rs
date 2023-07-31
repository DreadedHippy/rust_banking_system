use std::time::Duration;

use crate::{utils::{self, print_prompt, save_customer, read_database, prompt, get_int_input, overwrite_db, goto_main_menu, empty_line, yes_or_no_decistion}, models::{Customer, Account}, main};

const ADMINPASSWORD: &str = "adminadminadmin"; 


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

pub fn deposit_money() {
	let name = prompt("Enter your name: ");
	let pin_code = prompt("Please enter your PIN code for verification: ");

	let mut customers: Vec<Customer> = read_database(); // Read from the database

	// Search for customer
	let (customer_index, customer) = if let Some(c) = customers.iter().enumerate().find(|&(_, customer)| customer.name == name && customer.pin_code == pin_code) {
		c // If customer found, return customer
	} else {
		println!("Sorry, it seems you have not registered with us");
		std::thread::sleep(Duration::new(1, 0));

		// If customer not found, inquire if they would like to register and act on their choice accordingly
		match prompt("Would you like to register?(Y/N): ").to_uppercase().as_str() {
			"Y" => {println!("Yay! Let's get started then."); new_customer()},
			"N" => {print!("Alright. Thank you, have a nice day")},
			_ => {println!("I understand, have a nice day.")}
		};

		return
	};

	// Customer found
	if customer.accounts.len() > 0 { // If the customer has at least 1 account
		println!("Your account(s): ");
		for account in &customer.accounts { // list their accounts
			println!("  - {:?}", account);
		}
		empty_line();
		let selected_account_number = prompt("Select the account number of the account you would like to deposit into: "); // Prompt the customer to select an account
		if customer.accounts.iter().any(|acc| acc.account_number == selected_account_number) { // If the account number they selected is valid
			print_prompt("Enter the amount you would like to deposit: "); // Prompt them for an amount
			let selected_amount = get_int_input(None, 50000);

			let mut customer = customer.to_owned(); // Take ownership of the customer and deposit into their account
			match customer.deposit_into_account(selected_account_number, selected_amount) {
				Ok(new_balance) => {
					customers[customer_index] = customer; // Replace the customer in this index with the updated customer
					overwrite_db(customers); // Overwrite the db with this information
					println!("Your account has been credited successfully, your new balance is {}", new_balance);
					empty_line();
					goto_main_menu();  // go to main menu
				},
				Err(e) => println!("{}", e) // Print any errors to stdout
			}
		} else { // Selected account is invalid
			if yes_or_no_decistion("Could not find the account with the corresponding number, would you like to create an account? ") {
				print!("Alright ");
				create_account(customers, customer_index);
				return;
			}			
		}
	} else { // Customer does not have an account, ask if they would like to create one and act accordingly
		match prompt("No account found, would you like to create an account?").to_uppercase().as_str() {
			"Y" => {
				println!("Yay! Let's get started then.");				
				create_account(customers.to_owned(), customer_index)
			},
			"N" => {print!("Alright. Thank you, have a nice day")},
			_ => {println!("I understand, have a nice day.")}
		}

		return
	}
}


pub fn withdraw_money() {
	let name = prompt("Enter your name: ");
	let pin_code = prompt("Please enter your PIN code for verification: ");

	let mut customers: Vec<Customer> = read_database(); // Read from the database

	// Search for customer
	let (customer_index, customer) = if let Some(c) = customers.iter().enumerate().find(|&(_, customer)| customer.name == name && customer.pin_code == pin_code) {
		c // If customer found, return customer
	} else {
		println!("Sorry, it seems you have not registered with us");
		std::thread::sleep(Duration::new(1, 0));

		// If customer not found, inquire if they would like to register and act on their choice accordingly
		if yes_or_no_decistion("Would you like to register?(Y/N): ") {
			println!("Yay, let's get started then");
			new_customer();
			return
		}

		return
	};

	// Customer found
	if customer.accounts.len() > 0 { // If the customer has at least 1 account
		println!("Your account(s): ");
		for account in &customer.accounts { // list their accounts
			println!("  - {:?}", account);
		}
		empty_line();
		let selected_account_number = prompt("Select the account number of the account you would like to withdraw from: "); // Prompt the customer to select an account
		if customer.accounts.iter().any(|acc| acc.account_number == selected_account_number) { // If the account number they selected is valid
			print_prompt("Enter the amount you would like to withdraw: "); // Prompt them for an amount
			let selected_amount = get_int_input(None, 50000);

			let mut customer = customer.to_owned(); // Take ownership of the customer and deposit into their account
			match customer.withdraw_from_account(selected_account_number, selected_amount) {
				Ok(new_balance) => {
					customers[customer_index] = customer; // Replace the customer in this index with the updated customer
					overwrite_db(customers); // Overwrite the db with this information
					println!("Your account has been debited successfully, your new balance is {}", new_balance);
					empty_line();
					goto_main_menu();  // go to main menu
				},
				Err(e) => println!("{}", e) // Print any errors to stdout
			}
		} else { // Selected account is invalid
			if yes_or_no_decistion("Could not find the account with the corresponding number, would you like to create an account? ") {
				print!("Alright ");
				create_account(customers, customer_index);
				return;
			}			
		}
	} else { // Customer does not have an account, ask if they would like to create one and act accordingly
		if yes_or_no_decistion("No account found, would you like to create an account?") {
			println!("Yay! Let's get started then.");				
			create_account(customers.to_owned(), customer_index)
		}

		return
	}
}

pub fn get_account_balances() {
	let name = prompt("Enter your name: ");
	let pin_code = prompt("Please enter your PIN code for verification: ");

	let customers: Vec<Customer> = read_database(); // Read from the database

	// Search for customer
	let (customer_index, customer) = if let Some(c) = customers.iter().enumerate().find(|&(_, customer)| customer.name == name && customer.pin_code == pin_code) {
		c // If customer found, return customer
	} else { // Customer not found
		println!("Sorry, it seems you have not registered with us");
		std::thread::sleep(Duration::new(1, 0));

		// Inquire if they would like to register and act on their choice accordingly
		if yes_or_no_decistion("Would you like to register?(Y/N): ") {
			println!("Yay, let's get started then");
			new_customer();
			return
		}

		return
	};

	// Customer found
	if customer.accounts.len() > 0 { // If the customer has at least 1 account
		println!("Your balance(s): ");
		for account in &customer.accounts { // list their accounts
			println!("  - {:?}", account);
		}
		empty_line();
		goto_main_menu()
	} else { // Customer does not have an account, ask if they would like to create one and act accordingly
		if yes_or_no_decistion("No account found, would you like to create an account?") {
			println!("Yay! Let's get started then.");				
			create_account(customers.to_owned(), customer_index)
		}
		return
	}
}


fn create_account(customers: Vec<Customer>, customer_index: usize) {
	let mut customers = customers;
	let account_type = loop {
		match prompt("Select account type(C/S): ").to_uppercase().as_str() {
			"C"	=> break String::from("current"),
			"S" => break String::from("savings"),
			_ => println!("Invalid input")
		}
	};
	println!("Input a 4-digit account number");
	let account_number= get_int_input(Some(1000), 9999).to_string();
	let account: Account = Account { account_number: account_number.clone(), account_type, balance: 0 };

	let index = customer_index;
	let existing_accounts = &customers[index].accounts;

	if existing_accounts.len() > 0 {
		for i in 0..existing_accounts.len() {
			if existing_accounts[i].account_number == account_number{
				println!("An account with the given account number already exists");
				return;
			}
		}
	}
	
	customers[customer_index].accounts.push(account);

	overwrite_db(customers);
	println!("\n Account saved! Thank you \n");
	std::thread::sleep(Duration::new(1, 0));
	goto_main_menu();
	main()
}

pub fn get_admin_info() {
	if prompt("Enter admin credentials: ") == ADMINPASSWORD {
		println!("Access granted, getting customers");
		std::thread::sleep(Duration::new(1, 0));
		let customers: Vec<Customer> = read_database(); // Read from the database
		println!("{:?}", customers)
	} else {
		println!("Access denied");
		return
	}
}