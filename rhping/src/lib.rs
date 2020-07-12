use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test() {
        assert_eq!(1, 0);
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Main logic goes here.
    Ok(())
}


pub struct Config {
    pub url: String,
    pub timeout: u32,
    pub output_resp: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 2 {
            return Err("Usage: rhping URL [-t=timeout] [-o]");
        }

        let url = args[1].clone();
        let timeout = 0;
        let output_resp = false;

        Ok(Config { url, timeout, output_resp })
    }
}

