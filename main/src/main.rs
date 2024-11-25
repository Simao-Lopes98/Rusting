use core::num;
use std::usize;

/*  
*
* Ownership - Varaibles are responsible for freeing their own resources
* Stack and Heap Memory
*
*  
*
*/
fn main(){
    
    let rocket_fuel: String = String::from("RP-1");
    let ret: u8 = process_fuel(rocket_fuel.clone()); //Without clone. Ownership will be lost
    println!("Rocket fuel {rocket_fuel}");

    
}

fn process_fuel(propellant: String) -> u8 {
    println!("Processing propellant {} ...", propellant);
    return 15;
}
