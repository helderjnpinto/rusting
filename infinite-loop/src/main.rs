fn main() {
    let mut n = 0;
    loop {
        n += 1;

        if n < 10 {
            // not execute any below this line
            continue;
        } else if n > 50 {
            println!("Else if Rust finish!");
            break;
        } else {
            println!("Else Rust!");
        }

        println!("Rust line {} !", n);
    }
}
