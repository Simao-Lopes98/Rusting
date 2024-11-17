/*  
*
*
* MUltidim Arrays
*
*
*/
fn main() {


    let parking_lot: [[u8; 3]; 2] = [[1, 2, 3] 
                                    , [4 ,5 ,6]];

    let number:u8 = parking_lot[1][2];

    println!("Number is {number}");

    let garage: [[[i32; 100]; 20]; 5] = [[[0;100]; 20]; 5]; //3D array initialiez with zeros

    }