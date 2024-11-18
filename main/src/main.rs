use core::f32;
use std::result;

/*  
*
*
* Functions
*
*
*/
fn main() {
    let celsius_temp: f64 = 23.0;
    let fhar_temp: f64 = celsius_to_fah(celsius_temp);

    println!("Converted temp: {}", fhar_temp);
    }

fn celsius_to_fah(cesl:f64) -> f64{
    return ((1.8 * cesl) + 32 as f64);
}
