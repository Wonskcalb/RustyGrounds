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
    // TODO There should be a way to avoid so much duplication
    let verb: String = read_from_user("Enter a verb");
    let noun: String = read_from_user("Enter a noun");
    let adverb: String = read_from_user("Enter an adverb");
    let adjective: String = read_from_user("Enter an adjective");

    println!(
        "Do you {} your {} {} {}? That's hilarious!",
        verb, adjective, noun, adverb
    );
}
