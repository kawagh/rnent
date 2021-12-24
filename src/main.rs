extern crate chrono;
extern crate clap;
extern crate glob;
extern crate regex;
use chrono::Local;
use glob::glob;
use regex::Regex;
use std::{fs::File, io::BufRead, io::BufReader, io::Write};

use clap::{App, Arg};

fn extract_line(
    output_file: &mut File,
    input_filename: &str,
    tag: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // pattern to change context
    let re_general_tag = Regex::new(r"^# .*").unwrap();

    // pattern to extract following lines
    let tag_source = r"^# ";
    let tag_source = tag_source.to_string() + tag;
    let re_target_tag = Regex::new(&tag_source).unwrap();

    let mut is_in_target_context = false;
    for line in BufReader::new(File::open(input_filename)?).lines() {
        let l = line?;
        match is_in_target_context {
            true => {
                if re_target_tag.is_match(&l) {
                    continue;
                } else if re_general_tag.is_match(&l) {
                    is_in_target_context = false;
                    continue;
                }
                if let Err(e) = writeln!(output_file, "{}", l) {
                    println!("Writing error: {}", e.to_string());
                }
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
    let matches = App::new("rnent")
        .version("1.0")
        .author("kawagh")
        .about("Tool to write note")
        .arg(
            Arg::with_name("synthe")
                .help("synthe by the tag")
                .short("s")
                .takes_value(true),
        )
        .get_matches();
    // TODO added subcommand to create files

    if let Some(tag) = matches.value_of("synthe") {
        println!("the tag: {}", tag);

        let utc_date = Local::today().format("%Y-%m-%d");
        let mut output_file = File::create(utc_date.to_string() + ".md")?;
        output_file.write_all(String::from("new\n").as_bytes())?;
        let mut files: Vec<std::path::PathBuf> = glob("./test_resources/*.md")
            .unwrap()
            .filter_map(Result::ok)
            .collect();
        files.sort();

        for f in &files {
            if let Err(e) = writeln!(output_file, "{}", f.display()) {
                println!("Writing error: {}", e.to_string());
            };
            println!("{}", f.display());
            extract_line(&mut output_file, f.to_str().unwrap(), &tag.to_string()).ok();
        }
        println!("{:?}", files);

        println!("{}", utc_date);
    }

    Ok(())
}
