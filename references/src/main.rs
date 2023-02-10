fn main() {
    let mut x = 10;
    // let xr = &x;
    // println!("X is {}", xr);

    {
        let dom = &mut x;
        println!("dom is {}", dom);

        *dom += 50;
    }

    
    println!("final x is {}", x);
}
