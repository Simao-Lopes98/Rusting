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

    loop { // Infinite loop -> CTRL-C to stop
        count += 1;
        println!("Count {count}");

    }

}
