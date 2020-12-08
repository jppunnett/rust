use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};

use reqwest::blocking::Client;
use structopt::StructOpt;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn some_test() {
        assert_eq!(1, 0);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Pinging {:?} with timeout {:?}...",
        config.url, config.timeout
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(config.timeout))
        .build()?;

    let mut tot_resp_time: Duration = Duration::from_millis(0);
    for _x in 0..5 {
        let resp_time = http_ping(&client, &config.url)?;
        tot_resp_time += resp_time;
        println!("Reply from {}: time: {:?}", config.url, resp_time);
        sleep(Duration::from_millis(1000));
    }

    println!("Avg response time: {:?}.", tot_resp_time / 5);

    Ok(())
}

fn http_ping(client: &Client, url: &str) -> Result<Duration, Box<dyn Error>> {
    let now = Instant::now();
    client.head(url).send()?;
    Ok(now.elapsed())
}

#[derive(StructOpt)]
#[structopt(about = "Ping a URL.")]
pub struct Config {
    #[structopt(help = "URL to ping.")]
    pub url: String,

    #[structopt(default_value = "500", help = "Timeout in milliseconds.")]
    pub timeout: u64,
}
