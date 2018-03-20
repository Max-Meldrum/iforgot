extern crate clap;
extern crate toml;
#[macro_use]
extern crate serde_derive;

use std::fs;
use std::path::Path;
use std::env;
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
                 .help("Search key"))
        .get_matches();

    match init() {
        Some(path) => {
            let key = matches.value_of("KEY").unwrap().to_string();
            let config = get_lost_memory(path);
            let memories = analyze_memory(key, &config.memories);
            for mem in &memories {
                println!("{}", mem.fmt())
            }
        }
        None => println!("Failed to fetch config")
    }
}

fn init() -> Option<String> {
    match env::home_dir() {
        Some(path) => {
            let mut iforgot_path = path.display()
                .to_string()
                .to_owned();
            iforgot_path.push_str("/.iforgot");

            let exists = Path::new(&iforgot_path).exists();
            fs::create_dir_all(&iforgot_path).expect("Could not create iforgot directory!");
            iforgot_path.push_str("/iforgot.toml");
            if !exists {
                File::create(&iforgot_path).expect("Could not create config!");
            }
            Some(iforgot_path)
        }
        None => {
            println!("Hmm, your OS is not supported!");
            None
        }
    }
}

fn get_lost_memory(path: String) -> Config {
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

