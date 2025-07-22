fn main() {
    let mut x: u8 = 255;
    // x = x + 1; -> will cause an ovfl
    println!("x is {}", x);
    x = 20; // Has to be declared as mut
    println!("x is {}", x);

    let mut b_value = 0b1111_0101u8;
    println!("value is {:08b}", b_value); // Print binary

    b_value = !b_value; //Bitwise NOT

    b_value &= 0b1111_0111; // Bitwise AND

    println!("bit 6 is {}", b_value & 0b0100_0000);

    b_value |= 0b0100_0000; // Bitwise OR

    b_value ^= 0b0001_0000; // Bitwise XOR

    b_value = b_value << 4; // Left-Bitshift
    b_value = b_value >> 4; // Right-Bitshift

    let letter = 'a';
    let emoji = '😁';

    println!("{}, {}", letter, emoji);
}
