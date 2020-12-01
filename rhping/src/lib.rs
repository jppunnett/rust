use std::error::Error;
use std::thread::{sleep};
use std::time::{Duration, Instant};

use reqwest::blocking::Client;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn some_test() {
        assert_eq!(1, 0);
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!("Pinging {:?} with timeout {:?}...", config.url, config.timeout);

    let client = reqwest::blocking::Client::builder()
        .timeout(config.timeout)
        .build()?;

    for _x in 0..5 {
        http_ping(&client, &config.url)?;
        sleep(Duration::from_millis(1000));
    }

    Ok(())
}

fn http_ping(client: &Client, url: &String) -> Result<(), Box<dyn Error>> {

    let now = Instant::now();

    client.head(url).send()?;

    println!("Received response in {:?}", now.elapsed());

    Ok(())
}


pub struct Config {
    pub url: String,
    pub timeout: Duration,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 2 {
            return Err("Usage: rhping URL [timeout]");
        }

        let url = args[1].clone();
        let timeout = if args.len() > 2 {
            match args[2].parse() {
                Ok(t) => Duration::from_millis(t),
                Err(_) => return Err("Are you sure timeout is a number?"),
            }
        }
        else { Duration::from_millis(500) };

        Ok(Config { url, timeout })
    }
}

