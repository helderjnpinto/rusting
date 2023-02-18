fn main() {
    let name = String::from("Helder");
    match name.chars().nth(3) {
        Some(c) => println!("{}", c),
        _ => println!("Nothing"),
    }
}
