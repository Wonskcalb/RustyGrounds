use chrono::prelude::*;
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

fn request_int_from_user(prompt: &str) -> i32 {
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

fn main() {
    let age = request_int_from_user("What is your age?");
    let retirement = loop {
        let retirement = request_int_from_user("At what age would you like to retire?");

        if age <= retirement {
            break retirement;
        }

        println!("You would already be retired! Please try again");
    };

    let local: Date<Local> = Local::today();
    let delta = retirement - age;

    println!("You have {} years left until you can retire.", delta);
    println!(
        "It's {}, so you can retire in {}",
        local.year(),
        local.year() + delta
    );
}
