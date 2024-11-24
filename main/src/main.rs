use core::num;
use std::usize;

/*  
*
*
* Conditions & Loops
*
*
*/
fn main() {

    let mut count= 0;
    let letters= ['a','b','c'];

    while count < letters.len() {
        println!("Letters is {}", letters[count]);
        count += 1;
    }

    let message: [char; 5] = ['h','e','l','l','o'];

    for (index,&item) in message.iter().enumerate() {//Returns topple
        println!("item {} is {}", index, item);
        if item == 'e'{
            break;
        }
    }

    for number in 0..5 {
        println!("{}", number);
    }
}
