extern crate requests;

use gist_cli;

static URI: &'static str = "https://api.github.com/gists";

fn main() {
    gist_cli::parse_args(std::env::args());
}
