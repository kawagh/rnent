use chrono::Local;
use glob::glob;
use regex::Regex;
use std::{fs::read_to_string, fs::File, io::Write};

pub fn do_synthe(tag: &str, ment_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
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
