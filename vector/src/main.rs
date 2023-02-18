fn main() {
    // let vec: Vec<i32> = Vec::new()
    let mut vec = vec![1, 2, 3];

    println!("{}", vec[2]);

    vec.push(49);

    println!("{}", vec[3]);

    vec.remove(2);
    println!("{}", vec[2]);

    for number in vec.iter() {
        println!("loop - {}", number);
    }
}
