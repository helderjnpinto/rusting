struct Rectangle {
    width: u32,
    height: u32
}


impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let rec = Rectangle { width: 19, height: 5 };
    rec.print_description();

    println!("Rectangle is square {}", rec.is_square());
}
