use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Create a photomosaic.")]
pub struct Config {
    /// Source image.
    #[structopt(parse(from_os_str))]
    pub source: PathBuf,
}

pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
