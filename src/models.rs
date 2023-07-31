use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
	pub name: String,
	pub pin_code: String,
	pub accounts: Vec<Account>
}

impl Customer {
	pub fn deposit_into_account(&mut self, account_number: String, amount: u32) -> Result<u32, String> {
		let (chosen_account_index, _) = self.accounts.iter().enumerate().find(|(_, account)| account.account_number == account_number).unwrap();
		let account = self.accounts.get_mut(chosen_account_index).unwrap();

		if let Ok(_) = account.deposit(amount) {
			Ok(account.balance)
		} else {
			Err(String::from("An error occurred while trying to deposit"))
		}
	}

	pub fn withdraw_from_account(&mut self, account_number: String, amount: u32) -> Result<u32, String> {
		let (chosen_account_index, _) = self.accounts.iter().enumerate().find(|(_, account)| account.account_number == account_number).unwrap();
		let account = self.accounts.get_mut(chosen_account_index).unwrap();

		if let Ok(_) = account.withdraw(amount) {
			Ok(account.balance)
		} else {
			Err(String::from("You cannot withdraw more than your balance"))
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
	pub account_number: String,
	pub account_type: String,
	pub balance: u32
}

impl Account {
	fn deposit(&mut self, amount: u32) -> Result<bool, bool> {
		self.balance+= amount;
		Ok(true)
	}
	
	fn withdraw(&mut self, amount: u32) -> Result<bool, bool> {
		if amount > self.balance {
			return Err(false);
		}
		self.balance-= amount;
		Ok(true)
	}
}

