
fn main() {
    let celsius = 23.0;
    let fah = celsius_to_fah (celsius);

    assert_eq!(fah, 73.4);

    println!("PASS");
}

fn celsius_to_fah (celsius: f64) -> f64 {
    return (1.8 * celsius) + 32.0;
}
