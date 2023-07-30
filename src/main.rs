use std::io::Write;
use crate::handlers::{new_customer, deposit_money};
// use crate::models::{Customer, Account};

mod utils;
mod models;
mod statics;
#[macro_use]
mod handlers;


fn main() {
    println!("RUSTLANG BANK MANAGEMENT SYSTEM");
    println!("1. Register");
    println!("2. Deposit Money");
    println!("3. Withdraw Money");
    println!("4. Balance Enquiry");
    println!("5. All bank account holder list");
    println!("6. Close a bank account");
    println!("7. Update a bank account");
    println!("8. Settings");
    println!("9. Exit");
    println!();
    start();
}

fn start() {
    print!("Select an option < 1-8>: ");
    std::io::stdout().flush().unwrap();
    match utils::get_int_input(8) {
        1 => {
            new_customer();
        },
        2 => {
            deposit_money();
        },
        3 => {},
        4 => {},
        5 => {},
        6 => {},
        7 => {},
        8 => {},
        _ => {}
    }
    // println!("{}", input);
}
