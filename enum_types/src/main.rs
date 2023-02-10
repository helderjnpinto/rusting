enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let direction:Direction = Direction::Up;

    match direction {
        Direction::Up => println!("Dog go up"),
        Direction::Down => println!("Dog go Down"),
        Direction::Left => println!("Dog go Left"),
        Direction::Right => println!("Dog go Right"),
    }

}
