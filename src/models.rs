use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
	pub name: String,
	pub pin_code: String,
	pub accounts: Vec<Account>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
	pub account_number: String,
	pub account_type: String,
	pub balance: i32
}

