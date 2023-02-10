fn main() {
    // let numbers = [1,2,3,4,5];
    let numbers: [i32; 5] = [1,2,3,4,5];

    println!("Num 5 {}", numbers[4]);

    for number in numbers.iter() {
        println!("{}", number);
    }


    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }

    let numbers2 = [2; 400]; // 2,2,2,...
    for i in 0..numbers2.len() {
        println!("{}", numbers2[i]);
    }
}
