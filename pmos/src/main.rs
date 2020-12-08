use std::process;

fn main() {
    if let Err(e) = pmos::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
