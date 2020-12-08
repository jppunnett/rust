use structopt::StructOpt;

use std::process;

fn main() {
    let config = rhping::Config::from_args();
    if let Err(e) = rhping::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
