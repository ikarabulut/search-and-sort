use std::io;
use std::io::Write;

fn get_i32(prompt: &str) -> i32 {
    println!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    trimmed.parse::<i32>()
        .expect("Error parsing integer")
}

fn main() {
    let length = get_i32("Please enter the number of items you want to sort:");
    let max_size = get_i32("Please enter the max length of the array to sort:");

    println!("{} length, {} size", length, max_size)
}
