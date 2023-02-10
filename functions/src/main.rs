fn main() {
    print_numbers_to(20);
    let is_even = is_even(10);
    println!("is even {}", is_even);
}

fn print_numbers_to(num: u32) {
    for n in 1..num {
        if is_even(n) {
            println!("{} is even", n);
        } else {
            println!("{} is odd", n);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}