const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // let mut missiles = 8;
    // let ready = 2;
    // println!("Firing {} of my {} missles", ready, missiles);
    // missiles = missiles - ready;
    // println!("{} missles left", missiles);

    // const STARTING_MISSILES: i32 = 8;
    // const READY_AMOUNT: i32 = 2;

    // let mut missiles = STARTING_MISSILES;
    // let ready = READY_AMOUNT;

    // println!("Firing {} of my {} missles", ready, missiles);
    // missiles = missiles - ready;
    // println!("{} missles left", missiles);

    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {} of my {} missles", ready, missiles);
    println!("{} missles left", missiles - ready);
}
