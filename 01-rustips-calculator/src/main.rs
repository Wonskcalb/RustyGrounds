use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn read_from_user(message: &str) -> String {
    let mut user_response = String::new();

    print!("{} ", message);

    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut user_response) {
        Ok(_) => return user_response,
        Err(_) => println!("Unable to read the input."),
    };

    return user_response;
}

fn request_float_from_user(prompt: &str) -> f32 {
    loop {
        let raw_bill: String = read_from_user(&prompt);

        match raw_bill.trim().parse() {
            Ok(value) => match (value as i32).cmp(&0) {
                Ordering::Greater => return value,
                _ => println!("You must enter a number greater than 0!"),
            },
            Err(_) => println!("You can only type in numbers!"),
        }
    }
}

fn compute_tax(amount: f32, tax: f32) -> f32 {
    let multiplier = tax / 100.0;

    return amount * multiplier;
}

fn main() {
    println!("RusTipCalculator");

    let bill_prompt = "Enter the bill total?";
    let tax_prompt = "What it the tax percentage?";

    let bill = request_float_from_user(bill_prompt);
    let tax_perc = request_float_from_user(tax_prompt);

    let tip = compute_tax(bill, tax_perc);

    println!("The tip is {}", tip);
    println!("The total is {}", bill as f32 + tip);
}
