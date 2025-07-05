// 🦀 Rust Data Types Deep Dive Practice
// Master the fundamentals before moving to advanced concepts!

#[allow(dead_code)]
pub fn integer_types_practice() {
    println!("=== INTEGER TYPES PRACTICE ===");
    
    // Signed integers (can be negative)
    let small_signed: i8 = -128;        // -128 to 127
    let medium_signed: i16 = -32_768;   // -32,768 to 32,767  
    let normal_signed: i32 = -2_000_000; // Default integer type
    let big_signed: i64 = -9_000_000_000_000_000_000; 
    let huge_signed: i128 = -170_000_000_000_000_000_000_000_000_000_000_000_000;
    let arch_signed: isize = -1000; // Size depends on architecture (32/64 bit)
    
    println!("Signed integers:");
    println!("i8: {}, i16: {}, i32: {}", small_signed, medium_signed, normal_signed);
    println!("i64: {}, i128: {}, isize: {}", big_signed, huge_signed, arch_signed);
    
    // Unsigned integers (only positive)
    let small_unsigned: u8 = 255;           // 0 to 255
    let medium_unsigned: u16 = 65_535;      // 0 to 65,535
    let normal_unsigned: u32 = 4_000_000;   // 0 to ~4 billion
    let big_unsigned: u64 = 18_000_000_000_000_000_000;
    let huge_unsigned: u128 = 340_000_000_000_000_000_000_000_000_000_000_000_000;
    let arch_unsigned: usize = 1000; // Used for array indexing
    
    println!("\nUnsigned integers:");
    println!("u8: {}, u16: {}, u32: {}", small_unsigned, medium_unsigned, normal_unsigned);
    println!("u64: {}, u128: {}, usize: {}", big_unsigned, huge_unsigned, arch_unsigned);
    
    // Number formats
    let decimal = 98_222;           // Decimal
    let hex = 0xff;                 // Hexadecimal
    let octal = 0o77;               // Octal
    let binary = 0b1111_0000;       // Binary
    let byte = b'A';                // Byte (u8 only)
    
    println!("\nNumber formats:");
    println!("Decimal: {}, Hex: {}, Octal: {}, Binary: {}, Byte: {}", 
             decimal, hex, octal, binary, byte);
}

#[allow(dead_code)]
pub fn floating_point_practice() {
    println!("\n=== FLOATING POINT PRACTICE ===");
    
    let single_precision: f32 = 3.14159265; // 32-bit float
    let double_precision: f64 = 3.141592653589793; // 64-bit float (default)
    
    println!("f32: {}, f64: {}", single_precision, double_precision);
    
    // Math operations
    let x = 2.5;
    let y = 1.5;
    
    println!("Addition: {} + {} = {}", x, y, x + y);
    println!("Subtraction: {} - {} = {}", x, y, x - y);
    println!("Multiplication: {} * {} = {}", x, y, x * y);
    println!("Division: {} / {} = {}", x, y, x / y);
    println!("Remainder: {} % {} = {}", x, y, x % y);
    
    // Special float values
    let infinity = f64::INFINITY;
    let neg_infinity = f64::NEG_INFINITY;
    let not_a_number = f64::NAN;
    
    println!("Infinity: {}, -Infinity: {}, NaN: {}", infinity, neg_infinity, not_a_number);
}

#[allow(dead_code)]
pub fn boolean_practice() {
    println!("\n=== BOOLEAN PRACTICE ===");
    
    let is_rust_awesome: bool = true;
    let is_javascript_better: bool = false;
    
    println!("Is Rust awesome? {}", is_rust_awesome);
    println!("Is JavaScript better? {}", is_javascript_better);
    
    // Boolean operations
    let and_result = is_rust_awesome && is_javascript_better;
    let or_result = is_rust_awesome || is_javascript_better;
    let not_result = !is_javascript_better;
    
    println!("AND: {} && {} = {}", is_rust_awesome, is_javascript_better, and_result);
    println!("OR: {} || {} = {}", is_rust_awesome, is_javascript_better, or_result);
    println!("NOT: !{} = {}", is_javascript_better, not_result);
    
    // Comparison operations
    let a = 5;
    let b = 10;
    
    println!("Comparisons with {} and {}:", a, b);
    println!("{} == {} is {}", a, b, a == b);
    println!("{} != {} is {}", a, b, a != b);
    println!("{} < {} is {}", a, b, a < b);
    println!("{} > {} is {}", a, b, a > b);
    println!("{} <= {} is {}", a, b, a <= b);
    println!("{} >= {} is {}", a, b, a >= b);
}

#[allow(dead_code)]
pub fn character_practice() {
    println!("\n=== CHARACTER PRACTICE ===");
    
    let letter: char = 'A';
    let emoji: char = '🦀';
    let unicode: char = '\u{1F980}'; // Unicode crab emoji
    let chinese: char = '中';
    let math: char = 'π';
    
    println!("Letter: {}, Emoji: {}, Unicode: {}, Chinese: {}, Math: {}", 
             letter, emoji, unicode, chinese, math);
    
    // Character operations
    println!("Is '{}' alphabetic? {}", letter, letter.is_alphabetic());
    println!("Is '{}' numeric? {}", letter, letter.is_numeric());
    println!("Is '{}' uppercase? {}", letter, letter.is_uppercase());
    println!("Lowercase of '{}': {}", letter, letter.to_lowercase().collect::<String>());
}

#[allow(dead_code)]
pub fn string_types_practice() {
    println!("\n=== STRING TYPES PRACTICE ===");
    
    // String slice (&str) - immutable reference to string data
    let string_slice: &str = "Hello, Rust!";
    let static_str: &'static str = "This lives for the entire program";
    
    // String - owned, growable string
    let mut owned_string: String = String::from("Hello");
    let another_string: String = "World".to_string();
    let created_string: String = String::new();
    
    println!("String slice: {}", string_slice);
    println!("Static string: {}", static_str);
    println!("Owned string: {}", owned_string);
    println!("Another string: {}", another_string);
    println!("Created string: '{}'", created_string);
    
    // String operations
    owned_string.push_str(", World!");
    owned_string.push('!');
    
    println!("After modification: {}", owned_string);
    println!("Length: {}", owned_string.len());
    println!("Is empty? {}", owned_string.is_empty());
    println!("Contains 'World'? {}", owned_string.contains("World"));
    
    // String formatting
    let name = "Rust";
    let version = 1.87;
    let formatted = format!("Welcome to {} version {}!", name, version);
    println!("Formatted string: {}", formatted);
}

#[allow(dead_code)]
pub fn array_practice() {
    println!("\n=== ARRAY PRACTICE ===");
    
    // Fixed-size arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros: [i32; 3] = [0; 3]; // [0, 0, 0]
    let mixed = [1, 2, 3, 4, 5];
    
    println!("Numbers array: {:?}", numbers);
    println!("Zeros array: {:?}", zeros);
    println!("Mixed array: {:?}", mixed);
    
    // Array access
    println!("First element: {}", numbers[0]);
    println!("Last element: {}", numbers[numbers.len() - 1]);
    
    // Array iteration
    print!("Forward iteration: ");
    for num in numbers.iter() {
        print!("{} ", num);
    }
    println!();
    
    print!("With index: ");
    for (index, value) in numbers.iter().enumerate() {
        print!("{}:{} ", index, value);
    }
    println!();
}

#[allow(dead_code)]
pub fn tuple_practice() {
    println!("\n=== TUPLE PRACTICE ===");
    
    // Tuples can hold different types
    let person: (String, i32, bool) = (String::from("Alice"), 30, true);
    let coordinates: (f64, f64) = (10.5, 20.3);
    let mixed_tuple = ("Rust", 2024, true, 3.14);
    
    println!("Person tuple: {:?}", person);
    println!("Coordinates: {:?}", coordinates);
    println!("Mixed tuple: {:?}", mixed_tuple);
    
    // Tuple destructuring
    let (name, age, is_active) = person;
    println!("Destructured - Name: {}, Age: {}, Active: {}", name, age, is_active);
    
    // Tuple access by index
    let point = (10, 20, 30);
    println!("Point - x: {}, y: {}, z: {}", point.0, point.1, point.2);
    
    // Unit tuple (empty tuple)
    let unit: () = ();
    println!("Unit tuple: {:?}", unit);
}

#[allow(dead_code)]
pub fn practice_exercises() {
    println!("\n=== PRACTICE EXERCISES ===");
    
    // Exercise 1: Type conversion practice
    let integer: i32 = 42;
    let float_from_int = integer as f64;
    let small_int = integer as i8; // Be careful with overflow!
    
    println!("Original: {}, As f64: {}, As i8: {}", integer, float_from_int, small_int);
    
    // Exercise 2: String manipulation
    let mut sentence = String::from("Rust is");
    sentence.push_str(" awesome");
    sentence.push('!');
    let upper_sentence = sentence.to_uppercase();
    
    println!("Modified sentence: {}", sentence);
    println!("Uppercase: {}", upper_sentence);
    
    // Exercise 3: Array and tuple combination
    let grades: [i32; 5] = [85, 92, 78, 96, 88];
    let student = ("John Doe", grades, true);
    
    println!("Student: {:?}", student);
    
    // Calculate average
    let sum: i32 = grades.iter().sum();
    let average = sum as f64 / grades.len() as f64;
    println!("Average grade: {:.2}", average);
    
    println!("\n🎯 TODO: Try these exercises:");
    println!("1. Create a function that converts Celsius to Fahrenheit");
    println!("2. Make a tuple that represents a book (title, author, pages, available)");
    println!("3. Create an array of your favorite numbers and find the largest");
    println!("4. Practice with different number bases (binary, hex, octal)");
}

// Helper function to demonstrate all practices
#[allow(dead_code)]
pub fn run_all_practices() {
    integer_types_practice();
    floating_point_practice();
    boolean_practice();
    character_practice();
    string_types_practice();
    array_practice();
    tuple_practice();
    practice_exercises();
}
