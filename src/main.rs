extern crate clap;
use std::{fs::File, io::BufRead, io::BufReader};

use clap::App;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let matches = App::new("rnent")
        .version("1.0")
        .author("kawagh")
        .about("Tool to write note")
        .get_matches();

    for line in BufReader::new(File::open("test.md")?).lines() {
        let l = line?;
        println!("{}", l);
    }
    Ok(())
}
