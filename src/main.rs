use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    print!("Query: {}", config.query);

    let contents = fs::read_to_string(config.filename)
    .expect("Something went wrong");

    print!("With text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config{
    let query: String = args[1].clone();
    let filename: String = args[2].clone();

    Config { query, filename }
}