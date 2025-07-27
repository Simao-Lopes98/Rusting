/* Function lesosns */
fn main() {
    say_hello();
    let x: u8 = 1;
    let y: u8 = 2;
    say_the_sum (x, y);
    let result = square_return(40);
    println! ("og is {}, square is {}", result.0, result.1);
    println! ("It can also be printed out as {:?}", result);
}

fn say_hello()  {
    println!("Hello!");
    say_a_number(13);
}

fn say_a_number (number :i32) {
    println! ("number is {}", number);
}

fn say_the_sum (a: u8, b: u8){
     println! ("sum is {}", a + b);
}

fn square (x :i32) -> i32 {
    x * x   // returns if it is the last statement
}

fn square_return (x :i32) -> (i32, i32) { // Tupple return
    return (x, x * x)   // returns if it is the last statement
}