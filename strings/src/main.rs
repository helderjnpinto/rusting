fn main() {
    let mut my_string = String::from("Hello my name is helder");

    println!("length {}", my_string.len());

    println!("String is empty {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("- {}", token);
    }

    println!(
        "Does the string contains name: '?' {} ",
        my_string.contains("name")
    );

    my_string.push_str("!!!!!");
    println!("{}", my_string)

    // let mut hello = String::from("Hello, ");

    // hello.push('w');
    // hello.push_str("orld!");

    // println!("{}", hello)
}
