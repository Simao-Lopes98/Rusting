use core::num;
use std::usize;

/*  
*
* References
*
*/
fn main(){
    
    let mut rocket_fuel: String = String::from("RP-1");
    let length: usize = process_fuel(&mut rocket_fuel); // Ownership is not lost
    println!("Rocket fuel {rocket_fuel} {length}");
   
}

fn process_fuel(propellant: &mut String) -> usize { // Mutable reference
    println!("Processing propellant {} ...", propellant);
    propellant.push_str(" is highly flammable");
    let length = propellant.len();
    return length;
}
