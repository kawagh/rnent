extern crate chrono;
extern crate clap;
extern crate glob;
extern crate regex;
use chrono::Local;
use glob::glob;
use regex::Regex;
use std::{fs::File, io::BufRead, io::BufReader, io::Write};

use clap::App;

fn extract_line(filename: &str, tag: &str) -> Result<(), Box<dyn std::error::Error>> {
    // pattern to change context
    let re_general_tag = Regex::new(r"^# .*").unwrap();

    // pattern to extract following lines
    let tag_source = r"^# ";
    let tag_source = tag_source.to_string() + tag;
    let re_target_tag = Regex::new(&tag_source).unwrap();

    let mut is_in_target_context = false;
    for line in BufReader::new(File::open(filename)?).lines() {
        let l = line?;
        match is_in_target_context {
            true => {
                if re_target_tag.is_match(&l) {
                    continue;
                } else if re_general_tag.is_match(&l) {
                    is_in_target_context = false;
                    continue;
                }
                println!("{}", l);
            }
            false => {
                if re_target_tag.is_match(&l) {
                    is_in_target_context = true;
                }
            }
        };
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _matches = App::new("rnent")
        .version("1.0")
        .author("kawagh")
        .about("Tool to write note")
        .get_matches();
    // TODO added subcommand to create files

    let utc_date = Local::today().format("%Y-%m-%d");
    let mut new_file = File::create(utc_date.to_string() + ".md")?;
    new_file.write_all(String::from("new\n").as_bytes())?;
    let mut files: Vec<std::path::PathBuf> = glob("./test_resources/*.md")
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    files.sort();

    for f in &files {
        println!("{}", f.display());
        extract_line(f.to_str().unwrap(), "a");
    }
    println!("{:?}", files);

    println!("{}", utc_date);
    Ok(())
}
