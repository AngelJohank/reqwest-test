use reqwest;
use std::{env, process};

fn main() {
    let args: Vec<_> = env::args().collect();

    let url = args.get(1).unwrap_or_else(|| {
        println!("Please provide an url");
        process::exit(1)
    });

    let response = reqwest::blocking::get(url)
        .expect("failed fetching url")
        .text()
        .unwrap();

    println!("Response: \n{}", response)
}
