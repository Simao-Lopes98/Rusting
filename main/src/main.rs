use core::num;
use std::usize;

/*  
*
* References
*
*/
fn main(){
    
    let rocket_fuel: String = String::from("RP-1");
    let length: usize = process_fuel(&rocket_fuel); // Ownership is not lost
    println!("Rocket fuel {rocket_fuel} {length}");
   
}

fn process_fuel(propellant: &String) -> usize { // & borrowing
    println!("Processing propellant {} ...", propellant);
    let length = propellant.len();
    return length;
}
