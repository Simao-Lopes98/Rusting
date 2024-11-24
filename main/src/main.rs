use core::num;
use std::usize;

/*  
*
*
* Cahllenge
*
*
*/
fn main() {

    let numbers: [i32 ; 14] = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;   
    let mut min: i32;   
    let mut mean: f64;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    for num in numbers {
        if max < num {
            max = num;
        }
        if min > num{
            min = num;
        }
        mean += num as f64; // Add all the numebrs
    }

    mean = mean/numbers.len() as f64; // Calculate mean

    assert_eq!(max, 56);   
    assert_eq!(min, -18);   
    assert_eq!(mean, 12.5);   
    println!("Test Passed!");   

}
