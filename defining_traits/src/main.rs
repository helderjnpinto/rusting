struct Person {
    name: String,
    age: u8,
}

trait HasVoiceBox {
    fn speak(&self);
    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person {
    fn speak(&self) {
        println!("Hello, my name is {}.", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true;
        }
        return false;
    }
}

fn main() {
    let person = Person {
        name: String::from("Helder"),
        age: 34,
    };

    println!("Can this person speak {}", person.can_speak());

    if person.can_speak() {
        person.speak()
    }
}
