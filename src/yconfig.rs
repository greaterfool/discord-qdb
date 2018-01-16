/* Read configuration from YAML file */

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::{YamlLoader, YamlEmitter};

pub struct Config {
    pub token: String
}

pub fn read_config(input: &str) -> Config {
   let path = Path::new(input);
   let display = path.display();

   let mut file = match File::open(&path) {
       Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
       Ok(file) => file,
   };

   let mut contents = String::new();
   file.read_to_string(&mut contents)
       .expect("Something went wrong while reading the file.");

   // let mut confile = File::open(input.to_string()).expect("config.yaml not found.");

   // let mut contents = String::new();
   // confile.read_to_string(&mut contents).unwrap();

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];

    println!("{:?}", docs); // Debug support

    let mut out_str = String::new(); {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
    }
    println!("{}", out_str); // Debug support
    println!("{:?}", doc["Token"].as_str()); // Debug support

    let t = doc["Token"].as_str()
    //let t = out_str;
        .expect("Expected a token in config file.");

    Config { token: t.to_string() }
}
