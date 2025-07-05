// 🦀 Rust Ownership & Borrowing Practice
// This is crucial for understanding memory management in Rust

#[allow(dead_code)]
pub fn ownership_basics() {
    println!("=== OWNERSHIP BASICS ===");
    
    // 1. Basic ownership transfer
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("{}", s1); // This would cause an error!
    println!("s2: {}", s2);
    
    // 2. Copy types (integers, booleans, etc.)
    let x = 5;
    let y = x; // x is copied, both x and y are valid
    println!("x: {}, y: {}", x, y);
}

#[allow(dead_code)]
pub fn borrowing_basics() {
    println!("\n=== BORROWING BASICS ===");
    
    let s = String::from("hello world");
    
    // Immutable borrowing
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
    
    // Mutable borrowing
    let mut s2 = String::from("hello");
    change_string(&mut s2);
    println!("Changed string: {}", s2);
}

// Function that borrows a string (doesn't take ownership)
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but since it's a reference, nothing happens

// Function that borrows a mutable string
fn change_string(s: &mut String) {
    s.push_str(", world!");
}

#[allow(dead_code)]
pub fn slice_practice() {
    println!("\n=== STRING SLICES ===");
    
    let s = String::from("hello world");
    
    let hello = &s[0..5];   // or &s[..5]
    let world = &s[6..11];  // or &s[6..]
    let whole = &s[..];     // entire string
    
    println!("hello: {}", hello);
    println!("world: {}", world);
    println!("whole: {}", whole);
    
    // Finding first word
    let sentence = "The quick brown fox";
    let first_word = get_first_word(sentence);
    println!("First word: {}", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// Practice exercises for you to try!
#[allow(dead_code)]
pub fn practice_exercises() {
    println!("\n=== PRACTICE EXERCISES ===");
    
    // Exercise 1: Fix the ownership issue
    // Uncomment and fix this code:
    /*
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1); // This should work after you fix it
    */
    
    // Exercise 2: Write a function that takes a vector and returns the largest element
    let _numbers = vec![1, 5, 3, 9, 2];
    // let largest = find_largest(&numbers);
    // println!("Largest: {}", largest);
    
    // Exercise 3: Write a function that counts vowels in a string slice
    let _text = "Hello World";
    // let vowel_count = count_vowels(text);
    // println!("Vowels in '{}': {}", text, vowel_count);
    
    println!("TODO: Complete the exercises above!");
}

// TODO: Implement these functions as exercises
// fn find_largest(list: &[i32]) -> i32 {
//     // Your implementation here
// }

// fn count_vowels(s: &str) -> usize {
//     // Your implementation here
// }
