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
    
    let rocket_fuel: i32 = 1;
    let ret: u8 = process_fuel(rocket_fuel); // As int is in the stack it wont be modified
    println!("Rocket fuel {rocket_fuel}");

    
}

fn process_fuel(mut propellant: i32) -> u8 {
    propellant += 2;
    println!("Processing propellant {} ...", propellant);
    return 15;
}
