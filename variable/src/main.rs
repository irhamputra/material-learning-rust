// Global scope
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // mut for mutable value, let is immutable declared variable
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;

    println!("Firing {} of my {} missiles", ready, missiles);

    // missiles variable can be mutated
    missiles = missiles - ready;

    println!("{} missiles left", missiles);

}
