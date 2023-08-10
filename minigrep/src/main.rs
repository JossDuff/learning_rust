// cargo run -- to poem.txt
// searches for word "to" in poem.txt
// IGNORE_CASE=1 cargo run -- to poem.txt
// same as above but "to" is case insensitive
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
