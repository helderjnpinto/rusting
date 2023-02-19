extern crate reqwest;

#[tokio::main]
async fn main() {
    let response_text = reqwest::get("https://httpbin.org/get")
        .await
        .expect("Couldn't make request")
        .text()
        .await
        .expect("Could not read response text!");

    println!("Response: {}", response_text);

    // match reqwest::get("https://httpbin.org/get") {
    //     Ok(mut response) => {
    //         // check if is 200 ok
    //         if response.status() == reqwest::StatusCode::Ok {
    //             match response.text() {
    //                 Ok(t) => println!("{}", t),
    //                 Err(_) => println!("Could not read response text!"),
    //             }
    //         } else {
    //             println!("Response is not 200 Ok!");
    //         }
    //     }

    //     Err(_) => println!("Could not make the request!"),
    // }

    println!("Hello, world!");
}
