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
    let outer_planet: String;
    {
        let inner_planet: String = String::from("Mercury");
        println!("{inner_planet}");
        outer_planet = inner_planet; // Passing owernship
    }
    
    println!("{outer_planet}");
}
