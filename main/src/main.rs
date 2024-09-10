/*  
*
*
* Hello World Prog
*
*
*/
fn main() {
    let x: f32 = 10.0; // "mut" is used to let the compiler know that x can be reassigned. 
                          // ": u16" lets the compiler what type x is
    let y: u8 = 3;
    println!("x is {}, y is {}",x, y);
    let z: f32 = x / y as f32;
    println!("z is {}",z);
}