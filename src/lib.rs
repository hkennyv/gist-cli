pub struct Config;

pub fn parse_args(mut args: std::env::Args) -> Result<Config, &'static str> {
    // ignore self file name
    args.next();

    let mut filenames: Vec<String> = Vec::new();

    loop {
        let filename = match args.next() {
            Some(filename) => filenames.push(filename),
            None => break,
        };
    }

    println!("filenames: {:?}", filenames);

    Ok(Config {})

}
