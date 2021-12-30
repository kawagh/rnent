extern crate chrono;
extern crate clap;
extern crate glob;
extern crate regex;
use chrono::Local;
use glob::glob;
use regex::Regex;
use std::{env, fs, fs::File, io::BufRead, io::BufReader, io::Write};

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
                dbg!(&l);
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
// fn list_tags(base_path: &str) -> Vec<String> {
// }
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ment_dir = match env::var("MENT_DIR") {
        Ok(ment_dir) => ment_dir,
        Err(_) => {
            panic!("MENT_DIR is not set")
        }
    };
    let synthe_path = ment_dir.clone() + "/synthe";
    dbg!(&synthe_path);
    let flist = fs::read_dir(&synthe_path).unwrap();
    // let synthed_tags = flist.iter()
    for p in flist {
        println!("{:?}", p.unwrap().path().file_stem().unwrap());
    }

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

    if let Some(tag) = matches.value_of("synthe") {
        run_synthe(tag, &ment_dir).expect("synthe command failed");
    }
    // TODO added subcommand to create files

    Ok(())
}
