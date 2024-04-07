fn main() {
    // ---- ownership rules -----
    // 1. Each value in Rust has a variable thats called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will dropped.

    //     {
    //         //S in not valid here, its not yet declared
    //         let s = String::from("hello"); //s is valid from this point forword
    //                                        //do stuff with S
    //     } //this scope is now over, and is no longer valid

    //     // // How variable interact

    //     println!("Hello, world!");
    //     let s1 = String::from("hello");
    //     let s2 = s1.clone(); // Move to shallow copy

    //     println!("{}, world", s1);

    //     //function ownership
    //     let s = String::from("hello");
    //     takes_ownership(s);
    //     println!("{}", s);
    //
    let mut s = String::from("hello, world");
    let word = first_word(&s);
    s.clear();
}

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
