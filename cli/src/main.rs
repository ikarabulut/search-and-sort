mod args;

use std::io;
use std::io::Write;
use bubble_sort;
use utils::PseudoRandGen;

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

fn make_random_vec(max_size: i32, max: i32) -> Vec<i32> {
    let mut prng = PseudoRandGen::new();

    let mut vec: Vec<i32> = Vec::with_capacity(max_size as usize);
    for _ in 0..max_size {
        vec.push(prng.next_i32(0, max));
    }
    vec
}

fn main() {
    let args = args::Args::parse();
    let args::Args {
        sort_type
    } = args;

    let length = get_i32("Please enter the number of items you want to sort:");
    let max_size = get_i32("Please enter the highest allowed number in the list:");

    let vec_to_sort = make_random_vec(length, max_size);

    println!("Before Sort::");
    println!("{:?}", vec_to_sort);

    let sorted = match sort_type.as_str() {
        "bubble-sort" => bubble_sort::bubble_sort::sort(vec_to_sort),
        _ => unreachable!("Invalid sort type"), // This shouldn't happen due to validation
    };

    println!("After Sort::");
    println!("{:?}", sorted);
}
