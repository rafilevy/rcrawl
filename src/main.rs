extern crate clap;
use clap::{Arg, App};

mod rcrawl;
use crate::rcrawl::{Config, run};

fn u8_validator(s : String) -> Result<(), String> {
    match s.parse::<u8>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("The value must be a positive integer"))
    }
}

fn main() {
    let cl_matches = App::new("rcrawl")
        .version("1.0")
        .author("Rafi Levy. <rafilevy.co.uk>")
        .about("Recursively searches for a given file or directory and prints its full path to stdout")
        .arg(Arg::with_name("max_depth")
            .short("d")
            .long("max_depth")
            .takes_value(true)
            .help("The maximum depth to recursively search to")
            .validator(u8_validator)
        )
        .arg(Arg::with_name("single")
            .short("s")
            .long("single")
            .help("A flag indicating only the first found item should be output (equivalent to --max_items 1)")
        )
        .arg(Arg::with_name("max_items")
            .short("i")
            .long("max_items")
            .takes_value(true)
            .help("The maximum number of results to return")
            .validator(u8_validator)
        )
        .arg(Arg::with_name("dirname")
            .help("The name of the directory to change to")
            .required(true)
            .index(1)    
        )
        .get_matches();
        
    let config = Config::from_arg_matches(&cl_matches);
    
    match run(config) {
        Err(a) => eprintln!("Something went wrong: {}", a),
        _ => ()
    };
}
