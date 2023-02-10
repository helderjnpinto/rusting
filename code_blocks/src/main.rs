fn main() {
    let x = 10;

    {
        let y = 5;

        println!("x: {}, y: {}", x, y);
    }

    // will throw a error not find y
    // println!("after block: x: {}, y: {}", x, y);
}
