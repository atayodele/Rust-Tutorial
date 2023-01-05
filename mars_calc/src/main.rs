// use std::io;

fn main() {
    // let mut input = String::new(); // empty string 
    // io::stdin().read_line(&mut input);

    // println!("Input: {}", input);
    let mut mar_weight = calculate_weight_on_mars(100.0);
    mar_weight = mar_weight * 10000.0;
    println!("Weight on Mars: {}kg", mar_weight);
}
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}