fn main() {
    let name = String::from("Helder");

    println!(
        "Occupation is {}",
        match get_occupation(&name) {
            Some(o) => o,
            None => "No occupation found!",
        }
    )
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Helder" => Some("Software developer"),
        _ => None,
    }
}
