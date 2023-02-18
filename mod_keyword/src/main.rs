mod dcode {
    fn chicken() {
        println!("chicken");
    }

    pub fn print_msg() {
        println!("How its going!");
        chicken();
    }

    pub mod nested_module {
        pub fn print_msg() {
            println!("Nested !");
        }
    }
}

fn main() {
    {
        dcode::print_msg()
    }
    //  nested
    {
        dcode::nested_module::print_msg()
    }
}
