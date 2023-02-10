fn main() {
    let mut x = 10;



    {
        x = 50;

    }
    println!("x is 50: {}", x == 50);

    {
        let x = 5;

        println!("x: {}", x);
    }

    let x = "X is a string";
    println!("x is {}", x);


    let x = true;
    println!("x is {}", x);
}
