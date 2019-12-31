//! `cargo run --example blocking`
#![deny(warnings)]
use std::env;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let url = format!("http://{}", env::args().skip(1).next().expect("No URL provided"));

    println!("GET {}", url);

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let mut res = client.get(url.as_str()).send()?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());

    // copy the response body directly to stdout
    res.copy_to(&mut std::io::stdout())?;

    println!("\n\nDone.");
    Ok(())
}
