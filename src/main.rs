mod cli;

use std::env;
use std::fs;
use cli::CliConfig;
use std::process;
use std::error::Error;

/// ex: minigrep <inputfile> <pattern>
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = CliConfig::new(&args).expect("Something went wrong reading arguments");

    if let Err(e) = run(config) {
        println!("Error occurred: {}", e);
        process::exit(1);
    }
}

fn run(config: CliConfig) -> Result<(), Box<dyn Error>> {
    // read file
    let contents = fs::read_to_string(config.get_filename())?;

    // search for query
    let query = config.get_query();

    let found_lines: Vec<&str> = search(query, contents.as_str());

    for item in found_lines {
        println!("{}", item);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut found_lines: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            // The line contains query
            found_lines.push(line);
        }
    }

    found_lines
}
