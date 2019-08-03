extern crate serde;
extern crate serde_json;

use std::fs;
use std::collections::HashMap;
use serde::{Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize)]
struct GistJson {
    files: HashMap<String, Value>,
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

    Config { filenames, description, public }
}

pub fn build_json(config: &Config) -> String {
    let mut files: HashMap<String, Value> = HashMap::new();

    for filename in &config.filenames {
        let content_string = fs::read_to_string(filename).unwrap();
        let mut content = Map::new();
        content.insert("content".to_string(), Value::from(content_string));
        files.insert(filename.to_string(), Value::from(content));
    }

    let gist_json = GistJson {
        files: files,
        description: config.description.to_string(),
        public: config.public,
    };
    let j = serde_json::to_string(&gist_json).unwrap();
    println!("{}", j);
    return j;
}

