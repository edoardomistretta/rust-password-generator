use rand::Rng;
use std::{char, env};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let password_length: i32 = args[1].parse().unwrap();
            let random_chars = get_random_chars(password_length);
            println!("{}", random_chars)
        }
        _ => {
            let password_length = 24;
            let random_chars = get_random_chars(password_length);
            println!("{}", random_chars)
        }
    }
}

fn get_random_chars(chars_number: i32) -> String {
    let mut random_chars = String::new();
    let mut index = 0;
    while index < chars_number {
        random_chars.push(char::from_u32(rand::thread_rng().gen_range(33, 126)).unwrap());
        index += 1;
    }
    random_chars
}

