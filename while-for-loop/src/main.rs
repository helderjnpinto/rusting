fn while_loop() {
    let mut n = 1;

    while n <= 50 {
        if n % 5 == 0 {
          println!("N is multiple of 5 {}", n);
        }
        n +=1;
    }

}

fn main() {
    println!("Hello, world!");

    for i in 1..50 {
        if i % 5 == 0 {
          println!("N is multiple of 5 {}", i);
        }
    }

    // numbers is a variable and is a range is iterator
    let numbers = 30..51;
    for i in numbers {
        if i % 5 == 0 {
          println!("N is multiple of 5 {}", i);
        }
    }


    let animals = vec!["Rabbit", "Dog", "Cat"];


    /* 
    Also work without .iter() 
    for animala in animals.iter() {

    but pass the ownership and can be accessed after forloop
    */

    for animal in animals.iter() {
        println!("Animals {}", animal);
    }



    for (index, animal) in animals.iter().enumerate() {
        println!("Animals {} index is {}", animal, index);
    }
}
