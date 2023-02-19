use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arguments in args.iter() {
        println!("{}", arguments);
    }

    println!("{}", args[1]);
    println!("{}", args[2]);
}

// cargo run arg1 arg2
