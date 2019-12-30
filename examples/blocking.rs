//! `cargo run --example blocking`
#![deny(warnings)]
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let url = format!("http://{}", env::args().skip(1).next().expect("No URL provided"));

    println!("GET {}", url);

    let mut res = reqwest::blocking::get(url.as_str())?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());

    // copy the response body directly to stdout
    res.copy_to(&mut std::io::stdout())?;

    println!("\n\nDone.");
    Ok(())
}
