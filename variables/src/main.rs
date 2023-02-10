
fn immutable() {
    println!("Immutable");
    let x = 45;
    println!("The value of x is {}", x);
}

fn mutable() {
   println!("Mutable");
   // mut set X as Immutable variable
   let mut x = 45;
   println!("The value of x is {}", x);
   x = 60;
   println!("The value of x is {}", x);
}

fn variable_types() {
    let a = 50; // i32 rust dafaults to 32 bit integer 
   println!("a {}", a);

    let b: i64 = 50; // i64
   println!("b {}", b);

    let c: u64 = 50; // unsigned 
    println!("c {}", c);

    let d: f32 = 50.2; // float 32 bit
    println!("d {}", d);

    let e: bool = true; // float 
    println!("e {}", e);
}


fn main() {
   immutable();
   mutable();
   variable_types();
}
