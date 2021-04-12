use rand::Rng;
use std::char;
use std::io;

fn main() {
    let password_length = get_password_length();
    let random_chars = get_random_chars(password_length);

    println!("{}", random_chars)
}

fn get_random_chars(chars_number: usize) -> String {
    let mut random_chars = String::new();
    let mut index = 0;
    while index < chars_number {
        random_chars.push(char::from_u32(rand::thread_rng().gen_range(33, 126)).unwrap());
        index += 1;
    }
    random_chars
}

fn get_password_length() -> usize {
    println!("How long should the password be?");

    let mut password_length = String::new();
    io::stdin()
        .read_line(&mut password_length)
        .expect("Failed to read line");

    password_length.trim().parse().unwrap()
}
