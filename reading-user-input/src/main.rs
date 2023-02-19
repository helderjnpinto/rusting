use std::io;

fn main() {
    let mut input = String::new();

    println!("Hey type something: ");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Success: {}", input.to_uppercase());
        }

        Err(e) => println!("Oops! Something go wrong! {}", e),
    }
}
