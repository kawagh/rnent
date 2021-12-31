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
        do_synthe(tag, &ment_dir).expect("synthe command failed");
    }
    if let Some(_) = matches.subcommand_matches("update") {
        // update all synthesized tags
        do_update(&ment_dir);
        println!("update");
    }

    Ok(())
}
