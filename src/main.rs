//! gist-cli
//! author(s): khuynh
//! description: a cli tool that can create gists straight from your terminal.
//! consumes gist api v3:
//! https://developer.github.com/v3/gists/

extern crate reqwest;
extern crate rpassword;

use std::io::stdin;
use gist_cli;

static URI: &'static str = "https://api.github.com/gists";

fn main() {
    let config = gist_cli::parse_args(std::env::args());
    let json = gist_cli::build_json(&config);

    let mut username = String::new();
    println!("Enter your username:");
    stdin().read_line(&mut username).expect("Please enter a valid string:");
    username.pop();     // pop newline char

    let password =  rpassword::read_password_from_tty(
        Some("Enter your password:")).unwrap();

    let client = reqwest::Client::new();
    let res = client.post(URI)
        .basic_auth(username, Some(password))
        .body(json)
        .send().unwrap();

    println!("STATUS: {}", res.status());
}
