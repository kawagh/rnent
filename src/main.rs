extern crate chrono;
extern crate clap;
extern crate regex;
use chrono::Local;
use regex::Regex;
use std::{fs::File, io::BufRead, io::BufReader, io::Write};

use clap::App;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _matches = App::new("rnent")
        .version("1.0")
        .author("kawagh")
        .about("Tool to write note")
        .get_matches();

    // pattern to change context
    let re = Regex::new(r"^# .*").unwrap();

    // pattern to extract following lines
    let re_tag1 = Regex::new(r"^# tag1").unwrap();

    let mut is_in_target_context = false;
    // TODO added subcommand to create files
    // TODO list files
    for line in BufReader::new(File::open("test.md")?).lines() {
        let l = line?;
        match is_in_target_context {
            true => {
                if re_tag1.is_match(&l) {
                    continue;
                } else if re.is_match(&l) {
                    is_in_target_context = false;
                    continue;
                }
                println!("{}", l);
            }
            false => {
                if re_tag1.is_match(&l) {
                    is_in_target_context = true;
                }
            }
        };
    }
    let utc_date = Local::today().format("%Y-%m-%d");
    let mut new_file = File::create(utc_date.to_string() + ".md")?;
    new_file.write_all(String::from("new\n").as_bytes())?;
    println!("{}", utc_date);
    Ok(())
}
