struct Color {
    red: u8, // u8 0-255
    blue: u8,
    green: u8,
}

fn main() {
    let blue = Color {red: 0, green: 0, blue: 255};
    print_color(&blue);

    // when is not passed by reference after been used in function is dropped from the memory
    // print_color(blue);
}


// adding & we say is a reference of type color
fn print_color(c: &Color) {
    println!("Background color {},{},{}", c.red, c.green, c.blue);
}