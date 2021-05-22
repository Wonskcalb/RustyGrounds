use std::io;
use std::io::Write;

fn read_from_user(message: &str) -> String {
    let mut user_response = String::new();

    print!("{} ", message);

    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut user_response) {
        Ok(_) => return String::from(user_response.trim()),
        Err(_) => println!("Unable to read the input."),
    };

    return String::from(user_response);
}

fn main() {
    println!("== 03 Char counter == ");

    let word = read_from_user("What is the input string?");

    let len = word.len();
    println!(
        "'{}' has {} character{}",
        word,
        len,
        if len > 1 { "s" } else { "" }
    )
}
