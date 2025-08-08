fn main() {
    // println!("Hello, world!");
    
    /* 
    
    let mut st = String::from("starting to ");
    st.push_str(". code in rust ");
    println!("the string says {}" , st); 
    
    */
    
    
    /*
    
    let x = 10;
    let y = if x > 5 {"x is greater than 5 "} else {"x is less than 5"};
    println!("so y = {}" , y);
    
    */
    
    
    /*
    
    let num = 12 ; 
    match num {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        12 =>println!("twelve"),
        _ => println!("the number is smtg else"),
    }
    
    */
    
    
    /*

    let ans = divide(3.5,1.0);
    match ans {
    Ok(value) => println!("Result is {}", value),
    Err(e) => println!("Error: {}", e),
    }
    
    */

    
}

// fn divide(a:f64 , b:f64) -> Result<f64,String> {
//     if b == 0.0 {
//         Err(String::from("division with zero not valid"))
//     }
//     else {
//         Ok(a/b)
//     }
// }