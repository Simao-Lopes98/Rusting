use core::num;
use std::usize;

/*  
*
* References - Slices
*
*/
fn main(){
    
   let message: String = String::from("Greetings from Earth!");
   println!("Message is: {message}");

   let last_word: &str = &message[15..15+50];// Slice
   println!("Last word is {last_word}");

}