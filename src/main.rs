//! gist-cli
//! author(s): khuynh
//! description: a cli tool that can create gists straight from your terminal.
//! consumes gist api v3:
//! https://developer.github.com/v3/gists/
//!

extern crate requests;

use gist_cli;

static URI: &'static str = "https://api.github.com/gists";

fn main() {
    let config = gist_cli::parse_args(std::env::args());
    println!("{:?}", config);
    gist_cli::build_json(&config);
}
