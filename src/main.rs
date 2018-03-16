extern crate clap;
extern crate toml;
#[macro_use]
extern crate serde_derive;


use std::fs::File;
use std::io::prelude::*;
use clap::{Arg, App};
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Config {
    memories: Vec<Memory>
}

#[derive(Deserialize, Debug)]
struct Memory {
    name: String,
    tags: Vec<String>,
    commands: Vec<String>
}

fn main() {
    let matches = App::new("iforgot")
        .version("0.1.0")
        .author("Max Meldrum <max@meldrum.se>")
        .about("A little helper for the commands you always forget")
        .arg(Arg::with_name("KEY")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("Search term"))
        .get_matches();

    let _key = matches.value_of("KEY").unwrap();

    let config = get_lost_memory();
    println!("{:?}", config.memories);
}

fn get_lost_memory() -> Config {
    let path = "iforgot.toml";
    let mut file = File::open(&path).expect("Failed to open config file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed reading file");
    println!("{}", contents);
    toml::from_str(&contents).unwrap()
}

