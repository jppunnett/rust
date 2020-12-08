use structopt::StructOpt;
use std::process;

fn main() {
	let config = pmos::Config::from_args();
    if let Err(e) = pmos::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
