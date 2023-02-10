fn main() {
    let tup1 = (20, 25, 60);

    println!("Tuple 1 pos 2 {}", tup1.2);


    let tup2 = ("20", false, 60);

    println!("Tuple 2 pos 1 {}", tup2.1);


    let tup3 = ("20", false, 60, (1,"nested tuple",5));

    println!("Tuple 3 pos 1 {}", (tup3.3).1);

    // extract tuple
    let tup4 = ("20", false, 60);
    let (a,b,c) = tup4;

    println!("Tuple 4 pos 1 {}", 1);
}
