extern crate serde;
extern crate serde_json;

use std::fs;
use std::collections::HashMap;
use serde::{Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
struct GistJson {
    files: Value,
    description: String,
    public: bool,
}

#[derive(Debug, Serialize)]
/// A configuration struct for a gist. It contains the filenames of the files
/// to be uploaded to the gist, a gist description, and a boolean indicator
/// whether the gist will be public or private.
pub struct Config {
    pub filenames: Vec<String>,
    pub description: String,
    pub public: bool,
}



pub fn parse_args(mut args: std::env::Args) -> Config {
    // ignore self file name
    args.next();

    let mut filenames: Vec<String> = Vec::new();
    let mut public = true;
    let description = "This is a gist.".to_string();

    loop {
        match args.next() {
            Some(arg) => {
                match arg.as_str() {
                    "--private" => {
                        public = false;
                    },
                    _ => filenames.push(arg),
                }
            },
            None => break,
        };
    }

    println!("filenames: {:?}", filenames);

    Config { filenames, description, public }
}

pub fn build_json(config: &Config) {
    let files: HashMap<String, String> = HashMap::new();

    for filename in &config.filenames {
        println!("{}", filename);
    }

    let gist_json = GistJson {
        files: Value::from(5),
        description: config.description.to_string(),
        public: config.public,
    };
    let j = serde_json::to_string(&gist_json);
    println!("{:?}", j);
}

pub fn read_files() {
    // let mut filemap: HashMap<String, String> = HashMap::new();

    // for filename in &filenames {
    //     let content = fs::read_to_string(filename).unwrap();
    //     let content = match fs::read_to_string(filename)?;
    //     filemap.insert(filename.to_string(), content);
    // }

    // println!("filenames: {:?}", filenames);
    // println!("filemap: {:?}", filemap);
}
