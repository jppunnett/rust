use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use image::io::Reader as ImageReader;

#[derive(StructOpt)]
#[structopt(about = "Create a photomosaic.")]
pub struct Config {
    /// Source image.
    #[structopt(parse(from_os_str))]
    pub source: PathBuf,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	// Rotate source image as a test.
	let original = ImageReader::open(config.source)?.decode()?;
	original.rotate90().save("new.png")?;
	
    Ok(())
}
