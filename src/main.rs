use std::io::Write;
use crate::handlers:: Events;
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
    println!("8. Exit");
    println!();
    start();
}

fn start() {
    print!("Select an option < 1-8>: ");
    std::io::stdout().flush().unwrap();
    match utils::get_int_input(Some(1), 9) {
        1 => {
            Events::new_customer();
        },
        2 => {
            Events::deposit_money();
        },
        3 => {
            Events::withdraw_money();
        },
        4 => {
            Events::get_account_balances();
        },
        5 => {
            Events::get_admin_info();
        },
        6 => {
            Events::close_bank_account()
        },
        7 => {
            Events::update_bank_account();
        },
        8 => {
            println!("Thank you for banking with us");
            return;
        },
        _ => {}
    }
    // println!("{}", input);
}
