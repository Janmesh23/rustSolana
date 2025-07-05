mod ownership_practice;
mod datatypes_practice;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

struct Person {
    name: String,
    age: u32,
}

fn main() {
    // println!("Hello, world!");

    // numbers 
    let x: i32 = 5;// 32-bit signed integer -2^31 to 2^31-1
    let y: f64 = 3.14;// 64-bit floating point number
    let z: u8 = 255;// 8-bit unsigned integer 0 to 255
    let w: u32 = 10;// 32-bit unsigned integer 0 to 2^32-1

    println!("x: {}, y: {}, z: {}, w: {}", x, y, z, w);

    // let x: i8 = 1234587654376543 ; // the literal `1234587654376543` does not fit into the type `i8` whose range is `-128..=127` 
    
    println!("x: {}", x);


    // boolean
    let is_male: bool = true; // boolean value
    let is_adult: bool = true; // boolean value        

    if is_adult {
        println!("Adult");
    } else {
        println!("Not an adult");
    }
    if is_male && is_adult{
        println!("Adult Male");
    }

    // strings
    let greeting : String = String :: from(" hello ");
    println!("{}", greeting);
    let greet = "hell naah";
  println!("{}", greet);

 // loops
    for i in 0..5 {
        println!("i: {}", i);
    }

    let mut j = 0;
    while j < 5 {
        println!("j: {}", j);
        j += 1;
    }

    // functions
    let sum = add(5, 10);
    println!("Sum: {}", sum);

    // structs
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Person: {}, Age: {}", person.name, person.age);

    // 🦀 DATATYPES PRACTICE SESSION
    println!("\n{}", "=".repeat(60));
    println!("🦀 RUST DATA TYPES PRACTICE SESSION");
    println!("{}", "=".repeat(60));
    
    datatypes_practice::run_all_practices();
    
    
    // Uncomment below to practice ownership instead:
    // ownership_practice::ownership_basics();
    // ownership_practice::borrowing_basics();
    // ownership_practice::slice_practice();
    // ownership_practice::practice_exercises();
    
}
