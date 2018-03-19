extern crate clap;
extern crate toml;
#[macro_use]
extern crate serde_derive;


use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;
use clap::{Arg, App};

#[derive(Deserialize, Debug)]
struct Config {
    pub memories: Vec<Memory>
}

#[derive(Deserialize, Debug)]
struct Memory {
    name: String,
    pub tags: Vec<String>,
    commands: Vec<String>
}

impl Memory {
    fn fmt(&self) -> String {
        let mut res = String::new();
        res.push_str(&self.name);
        res.push('\n');
        for cmd in &self.commands {
            res.push_str(cmd);
            res.push('\n');
        }
        res
    }
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

    let key = matches.value_of("KEY").unwrap().to_string();
    let config = get_lost_memory();
    let memories = analyze_memory(key, &config.memories);
    for mem in &memories {
        println!("{}", mem.fmt())
    }
}

fn get_lost_memory() -> Config {
    let path = "iforgot.toml";
    let mut file = File::open(&path).expect("Failed to open config file!");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed reading file");
    toml::from_str(&contents).unwrap()
}

fn analyze_memory(key: String, memories: &Vec<Memory>) -> Vec<&Memory> {
    let mut result = Vec::new();
    for x in 0..memories.len() {
        if memories[x].tags.contains(&key) {
            result.push(&memories[x])
        }
    }
    result
}

