use core::f32;

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

    let x:u8 = 1;
    let y:u8 = 2;
    say_sum(x, y);
    say_a_number(x as i32);
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