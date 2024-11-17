/*  
*
*
* Tuples
*
*
*/
fn main() {

    let mut stuff: (i32, f64, char) = (10, 3.14, 'x');
    let mut f_item: i32 = stuff.0;
    println!("{f_item}");

    stuff.0 += 3;
    f_item = stuff.0;
    println!("Updated f item: {f_item}");

    let (a, b, c) = stuff;


    }