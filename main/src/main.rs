/*  
*
*
* Conditions & Loops
*
*
*/
fn main() {
    let is_odd: bool = true;
    let x: i32 = if is_odd {1} else {2}; // One-line if
    let mut count: i32= 0;

    let ret: i32= loop { // Infinite loop
        if count == 10 {
            break count * 10; // This makes so that the loop turns into an expression
        }
        count += 1;
        println!("Count {count}");

    };

    println!("After the loop {ret}");

}
