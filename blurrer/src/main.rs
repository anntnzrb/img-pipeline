use clap::Parser;
use color_eyre::eyre::Result;
use img_utils::{apply_blur, process_image, ProcessConfig};

fn main() -> Result<()> {
    color_eyre::install()?;

    let config = ProcessConfig::parse();
    process_image(&config, apply_blur)?;

    println!("Image processed. Blur applied.");
    Ok(())
}
