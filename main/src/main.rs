/*  
*
*
* Hello World Prog
*
*
*/
fn main() {

    let mut letters: [char; 3] = ['a','b','c'];
    letters[0] = 'x';
    let f_letters: char = letters[0];

    println!("{}", f_letters);

    let numbers: [i32; 5]; //Init empty array of i32 with length 5
    numbers = [0;5]; //Fill the array wth 0
    let index: usize = numbers.len();
    println!("{}",numbers[index]);

    }