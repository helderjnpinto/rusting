extern crate rand;
use rand::Rng;

fn main() {
    let rand_number = rand::thread_rng().gen_range(1, 11); // 1 - 11

    println!("{}", rand_number);

    let rand_b = rand::thread_rng().gen_weighted_bool(25);
    println!("{}", rand_b);
}
