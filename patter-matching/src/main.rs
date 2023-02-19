fn main() {
    let number = 11;

    match number {
        1 => println!("1"),
        2 => println!("2"),
        3 | 4 => println!("3 or 4"),
        11..=22 => println!("range of 11 to 22"),
        _ => print!("Its default"),
    }

    let name = "jn";

    match name {
        "hp" => println!("hp"),
        "jn" => println!("jn"),
        _ => print!("Its default name"),
    }
}
