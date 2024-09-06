mod bubble_sort;

use std::io;
use std::io::Write;
use crate::bubble_sort::bubble_sort;
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
    let length = get_i32("Please enter the number of items you want to sort:");
    let max_size = get_i32("Please enter the highest allowed number in the list:");

    let mut vec_to_sort = make_random_vec(length, max_size);

    println!("Before Sort::");
    println!("{:?}", vec_to_sort);

    let sorted = bubble_sort(&mut vec_to_sort);
    println!("After Sort::");
    println!("{:?}", sorted);
}
