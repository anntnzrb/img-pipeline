use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use image::{ImageBuffer, Rgb};
use img_utils::{
    image_to_shared_memory, load_image, open_shared_memory, save_image, shared_memory_to_image,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::NamedTempFile;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input image file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output image file path
    #[arg(short, long)]
    output: PathBuf,
}

type ImageBufferPair = (ImageBuffer<Rgb<u8>, Vec<u8>>, ImageBuffer<Rgb<u8>, Vec<u8>>);

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let img = load_image(&cli.input)?;
    let (width, height) = img.dimensions();
    let half_height = height / 2;

    let (top_half, bottom_half) = split_image(&img, half_height);

    let top_half_shared_memory = NamedTempFile::new()?;
    let bottom_half_shared_memory = NamedTempFile::new()?;

    process_image_half(
        "blurrer",
        &top_half,
        &top_half_shared_memory,
        width,
        half_height,
    )?;
    process_image_half(
        "edger",
        &bottom_half,
        &bottom_half_shared_memory,
        width,
        height - half_height,
    )?;

    let final_img = combine_processed_halves(
        &top_half_shared_memory,
        &bottom_half_shared_memory,
        width,
        height,
        half_height,
    )?;

    save_image(&final_img, &cli.output)?;
    println!(
        "Image processed. Multiple filters applied. Result saved to {}",
        cli.output.display()
    );

    Ok(())
}

fn split_image(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, half_height: u32) -> ImageBufferPair {
    let (width, height) = img.dimensions();
    let top_half = ImageBuffer::from_fn(width, half_height, |x, y| *img.get_pixel(x, y));
    let bottom_half = ImageBuffer::from_fn(width, height - half_height, |x, y| {
        *img.get_pixel(x, y + half_height)
    });
    (top_half, bottom_half)
}

fn process_image_half(
    executable: &str,
    half: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    shared_memory: &NamedTempFile,
    width: u32,
    height: u32,
) -> Result<()> {
    let mut mmap = open_shared_memory(shared_memory.path(), half.as_raw().len())?;
    image_to_shared_memory(half, &mut mmap)?;
    process_half_image(executable, shared_memory.path(), width, height)
}

fn process_half_image(
    executable: &str,
    shared_memory: &Path,
    width: u32,
    height: u32,
) -> Result<()> {
    let executable_path = std::env::current_exe()?
        .parent()
        .ok_or_else(|| eyre!("Failed to get parent directory"))?
        .join(executable);

    let status = Command::new(&executable_path)
        .arg("--shared-memory")
        .arg(shared_memory)
        .arg("--width")
        .arg(width.to_string())
        .arg("-H")
        .arg(height.to_string())
        .status()?;

    if !status.success() {
        return Err(eyre!("{} processing failed", executable));
    }

    Ok(())
}

fn combine_processed_halves(
    top_half_shared_memory: &NamedTempFile,
    bottom_half_shared_memory: &NamedTempFile,
    width: u32,
    height: u32,
    half_height: u32,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let top_half_processed = shared_memory_to_image(
        &open_shared_memory(
            top_half_shared_memory.path(),
            (width * half_height * 3) as usize,
        )?,
        width,
        half_height,
    )?;
    let bottom_half_processed = shared_memory_to_image(
        &open_shared_memory(
            bottom_half_shared_memory.path(),
            (width * (height - half_height) * 3) as usize,
        )?,
        width,
        height - half_height,
    )?;

    let mut final_img = ImageBuffer::new(width, height);
    for y in 0..half_height {
        for x in 0..width {
            *final_img.get_pixel_mut(x, y) = *top_half_processed.get_pixel(x, y);
        }
    }
    for y in 0..(height - half_height) {
        for x in 0..width {
            *final_img.get_pixel_mut(x, y + half_height) = *bottom_half_processed.get_pixel(x, y);
        }
    }
    Ok(final_img)
}
