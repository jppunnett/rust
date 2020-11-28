use std::error::Error;
use core::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test() {
        assert_eq!(1, 0);
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(config.timeout))
        .build()?
        .head(&config.url)
        .send()?;

    Ok(())
}

pub struct Config {
    pub url: String,
    pub timeout: u64,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 2 {
            return Err("Usage: rhping URL [timeout]");
        }

        let url = args[1].clone();
        let timeout = if args.len() > 2 {
            match args[2].parse() {
                Ok(t) => t,
                Err(_) => return Err("Are you sure timeout is a number?"),
            }
        }
        else { 500 };

        Ok(Config { url, timeout })
    }
}

