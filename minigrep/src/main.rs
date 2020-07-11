use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);

    println!("Searching for {}\nin file {}\n", query, filename);

    let contents = fs::read_to_string(filename)
        .expect("Could not read file");

    println!("With text:\n{}", contents);

}

fn parse_config(args: &[String]) -> (&str, &str) {

    (&args[1], &args[2])

}

