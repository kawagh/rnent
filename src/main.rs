mod cmd_synthe;
mod cmd_update;
extern crate chrono;
extern crate clap;
extern crate glob;
extern crate regex;
use crate::cmd_synthe::do_synthe;
use crate::cmd_update::do_update;
use std::env;

use clap::{App, Arg, SubCommand};
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
        .subcommand(
            SubCommand::with_name("synthe")
                .about("synthesize from daily files by tag")
                .arg(Arg::with_name("tag").help("tag to extract").index(1)),
        )
        .subcommand(SubCommand::with_name("update").about("update all synthesized documents"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("update") {
        do_update(&ment_dir);
        println!("update");
    }
    if let Some(synthe_match) = matches.subcommand_matches("synthe") {
        let synthe_tag = synthe_match
            .value_of("tag")
            .expect("tag is needed to synthe");
        do_synthe(synthe_tag, &ment_dir).expect("synthe command failed");
    }

    Ok(())
}
