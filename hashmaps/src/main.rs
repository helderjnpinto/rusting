use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    marks.insert("Rust Programming", 96);
    marks.insert("Web development", 22);
    marks.insert("UX Design", 75);
    marks.insert("Professional Computing Studies", 45);

    println!("How many subjects have you studied? {}", marks.len());

    match marks.get("Rust Programming") {
        Some(mark) => println!("Value of rust? {}", mark),
        None => println!("Not found "),
    }

    // remove value

    marks.remove("Web development");

    match marks.get("Web development") {
        Some(mark) => println!("Value of dev? {}", mark),
        None => println!("Not found web dev"),
    }

    println!("Print all:");
    for (subject, mark) in &marks {
        println!("For {} you got {}", subject, mark);
    }

    println!("Hash map contains UX? {}", marks.contains_key("UX Design"));
}
