struct Color {
    red: u8, // u8 0-255
    blue: u8,
    green: u8,
}

struct ColorTuple (u8,u8,u8);

fn main() {
    let mut bg = Color {red: 255, green: 70, blue: 15};

    // can't change if not have mut keyword 
    bg.blue = 5;

    println!("Background color {},{},{}", bg.red, bg.green, bg.blue);



    // Tuple structs

    let mut red = ColorTuple(255,0,0);

    red.2 = 60;

    println!("Red color {},{},{}", red.0, red.1, red.2);

}
