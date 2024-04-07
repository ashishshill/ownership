// The rules of references
// 1. At any given time, you can have either one mutable reference
// or any number of immutable references.
//
// 2. references must always be valid

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len(); // len() returns the length of String
    length
}
