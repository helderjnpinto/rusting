extern crate regex;
use regex::Regex;

fn main() {
    // let re = Regex::new("\\d");
    {
        let re = Regex::new(r"\w{5}").unwrap();
        let text = "helder";
        println!("{}", re.is_match(text));
    }

    // capture
    {
        let re = Regex::new(r"(\w{5})").unwrap();
        let text = "helder";

        match re.captures(text) {
            Some(caps) => println!("Some {}", caps.get(0).unwrap().as_str()),
            _ => println!("not match"),
        }
    }

    // capture 2
    {
        let re = Regex::new(r"(\w{5})").unwrap();
        let text = "helder";

        match re.captures(text) {
            Some(caps) => println!("Some 2 {}", &caps[0]),
            _ => println!("not match 2"),
        }
    }
}
