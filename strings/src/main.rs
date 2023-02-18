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
    println!("{}", my_string);

    // let mut hello = String::from("Hello, ");

    // hello.push('w');
    // hello.push_str("orld!");

    // println!("{}", hello)

    // replace
    {
        let mut my_string_b = String::from("rust is fantastic").replace("rust", "js");
        println!("{}", my_string_b);
    }
    // lines
    {
        let mut my_string_b = String::from("rust \nis\n fantastic");
        for line in my_string_b.lines() {
            println!("[ {} ]", line);
        }
    }
    // split
    {
        let mut my_string_b = String::from("leave+a+like+if+you+enjoyed!");

        let tokens: Vec<&str> = my_string_b.split("+").collect();
        println!("index 2 - {}", tokens[2]);
    }
    // trim
    {
        let mut my_string_b = String::from("    my name helder    \n\r");

        println!("before {}", my_string_b);
        println!("after {}", my_string_b.trim());
    }
    // chars
    {
        let my_string_b = String::from("helder on rust");

        // get char at index
        // return Option
        match my_string_b.chars().nth(4) {
            Some(c) => println!("Char at index 4: {}", c),
            None => println!("Not found"),
        }

        println!("before {}", my_string_b);
    }
}
