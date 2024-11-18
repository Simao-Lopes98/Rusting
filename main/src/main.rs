use core::f32;
use std::result;

/*  
*
*
* Functions
*
*
*/
fn main() {
    say_hello();
    say_hello();
    let y: (i32, i32) = square(13);
    println!("Result {:?}", y);
    }

fn say_hello() { // Function declaration. Rust doesnt care if there is no prototype.
    println!("Hello");
    say_a_number(32);
}

fn say_a_number(number:i32){
    println!("number is {number}");
}

fn say_sum(a:u8, b:u8){
    let sum: u8 = a + b;
    println!("Sum {sum}");    
}

fn square(x:i32) -> (i32, i32) { // Return example
    println!("Squaring");
    //x * x // Returns if it is the last line.
    return (x, x * x);
}

