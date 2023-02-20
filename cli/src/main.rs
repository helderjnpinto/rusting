use std::process::Command;

fn main() {
    let mut cmd = Command::new("python3");

    cmd.arg("pythonScript.py");

    // exec command
    match cmd.output() {
        Ok(o) => {
            // stdout is a vector of bytes
            unsafe {
                println!("Result: {}", String::from_utf8_unchecked(o.stdout));
            }
        }

        Err(e) => {
            println!("there was an error! {}", e);
        }
    }
}
