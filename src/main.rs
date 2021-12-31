extern crate chrono;
extern crate clap;
extern crate glob;
extern crate regex;
use chrono::Local;
use glob::glob;
use regex::Regex;
use std::{env, fs, fs::read_to_string, fs::File, io::Write};

use clap::{App, Arg, SubCommand};

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
    let mut sbuf = String::new();
    for line in read_to_string(input_filename)
        .expect("could not open the file")
        .lines()
    {
        match is_in_target_context {
            true => {
                if re_target_tag.is_match(&line) {
                    continue;
                } else if re_general_tag.is_match(&line) {
                    is_in_target_context = false;
                    continue;
                }
                sbuf.push_str(line);
                sbuf.push_str("\n");
            }
            false => {
                if re_target_tag.is_match(&line) {
                    is_in_target_context = true;
                }
            }
        };
    }
    if !sbuf.is_empty() {
        writeln!(output_file, "{}", sbuf).expect("Writing error");
    }
    Ok(())
}
fn run_synthe(tag: &str, ment_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("the tag: {}", tag);

    let utc_date = Local::today().format("%Y-%m-%d");
    let mut output_file = File::create("synthe_".to_owned() + tag + &utc_date.to_string() + ".md")?;

    let glob_pattern = ment_dir.to_owned() + "/**/*.md";
    let mut files: Vec<std::path::PathBuf> = glob(&glob_pattern)
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    files.sort();

    for f in &files {
        extract_line(&mut output_file, f.to_str().unwrap(), &tag.to_string()).ok();
    }
    Ok(())
}

// update all synthesized tags
fn run_update(ment_dir: &str) {
    let synthe_path = ment_dir.clone().to_owned() + "/synthe";
    dbg!(&synthe_path);
    let flist = fs::read_dir(&synthe_path).unwrap();
    for p in flist {
        let _path = p.unwrap().path();
        let synthed_tag = _path.file_stem().unwrap().to_str().unwrap();
        println!("{}", synthed_tag);
        if synthed_tag == "week" {
            continue;
        }
        run_synthe(&synthed_tag, &ment_dir).expect("synthe command failed");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ment_dir = match env::var("MENT_DIR") {
        Ok(ment_dir) => ment_dir,
        Err(_) => {
            panic!("MENT_DIR is not set")
        }
    };

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
        .subcommand(SubCommand::with_name("update").about("update all synthesized documents"))
        .get_matches();

    // TODO added subcommand to create files
    if let Some(tag) = matches.value_of("synthe") {
        run_synthe(tag, &ment_dir).expect("synthe command failed");
    }
    if let Some(_) = matches.subcommand_matches("update") {
        // update all synthesized tags
        run_update(&ment_dir);
        println!("update");
    }

    Ok(())
}
