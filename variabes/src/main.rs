fn first_example_of_variable() {
    let missiles = 8;
    let ready = 2;

    println!("Firing {} of my {} missiles", ready, missiles);
}

// SECOND EXAMPLE FROM FIRST ABOVE
const STARTING_MISSILE: i32 = 10;
const READY_AMT: i32 = 2;

fn second_example_of_variable() {
    let _be_use = first_example_of_variable();

    let mut missiles = STARTING_MISSILE;
    let ready = READY_AMT;

    missiles = missiles - ready;
    println!("Second example => {} missiles left", missiles);
}

const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let _b2 = second_example_of_variable();

    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

    missiles = missiles - ready;      
    println!("{} missiles left", missiles);
}
