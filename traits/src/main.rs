struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and i am {}", self.name, self.age);
    }
}

fn main() {
    let dom = Person {
        name: String::from("Helder"),
        age: 34,
    };

    println!("{}", dom.to_string());
}
