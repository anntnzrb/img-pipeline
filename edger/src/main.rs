use clap::Parser;
use color_eyre::eyre::Result;
use img_utils::{apply_edge_detection, process_image, ProcessConfig};

fn main() -> Result<()> {
    color_eyre::install()?;

    let config = ProcessConfig::parse();
    process_image(&config, apply_edge_detection)?;

    println!("Image processed. Edge Detection applied.");
    Ok(())
}
