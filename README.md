# gist-cli
author: khuynh

## Description
This is a simple CLI tool to publish gists from your terminal. It was
originally written in Python, now rewritten in rust for fun. Still very much
WIP, lots of features I hope to add:

* gist token support vs basic auth
* fix filename handling /'s
* dynamic description
* add better options for publishing multiple files/folders
* return URL

## Installation
Clone the repository:

```
git clone https://github.com/hkennyv/gist-cli.git
cd gist-cli
```

Right now, this crate is not published on crates.io, so will have to build it
locally and then you may run the binary.

```
cargo build
```

Binary is located in `target/debug`.
