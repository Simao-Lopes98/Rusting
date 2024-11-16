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
    let z: f32 = x / y as f32; // use "as" to type cast
    println!("z is {z}");

    let mut dio_register:u8 = 0b000_1111;
    println!("Value DIO {dio_register:0b}");
    dio_register = !dio_register;
    println!("Not DIO {dio_register:0b}");
    dio_register |= 0b0100_1001;
    println!("Bitwise-OR DIO {dio_register:0b}");
    println!("Bitwise-AND DIO {:0b}", dio_register & !(1 << 5));
    // panic!(); // used to panic. program exits

    // Comparisons and boolean data type
    let a:  bool = true;
    let b:  bool = false;
    println!("a is {}. b is {}", a, b);
    println!("a LEES THAN b is {}", a < b);
    println!("a GREATER THAN b is {}", a > b);

    // Char data type. Char uses 4bytes
    let letter:char = 'a';
    let number:char = '1';
    let finger:char = '\u{261D}';

    println!("{letter}\n{number}\n{finger}");
}