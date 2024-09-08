use clap::Parser;
use color_eyre::eyre::{eyre, Result};
use image::{ImageBuffer, Rgb};
use memmap2::{MmapMut, MmapOptions};
use rayon::prelude::*;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

#[derive(Parser)]
pub struct ProcessConfig {
    /// Input image file path
    #[clap(short, long)]
    pub input: Option<PathBuf>,

    /// Output image file path
    #[clap(short, long)]
    pub output: Option<PathBuf>,

    /// Shared memory path (for coordinated mode)
    #[clap(short, long)]
    pub shared_memory: Option<PathBuf>,

    /// Image width (for coordinated mode)
    #[clap(short, long)]
    pub width: Option<u32>,

    /// Image height (for coordinated mode)
    #[clap(short = 'H', long)]
    pub height: Option<u32>,
}

pub fn load_image(path: &Path) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    image::open(path)?
        .to_rgb8()
        .try_into()
        .map_err(|e| eyre!("Failed to convert image: {}", e))
}

pub fn save_image(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, path: &Path) -> Result<()> {
    img.save(path)
        .map_err(|e| eyre!("Failed to save image: {}", e))
}

pub fn apply_blur(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let (width, height) = img.dimensions();
    let mut new_img = img.clone();

    new_img
        .par_chunks_mut(3)
        .enumerate()
        .for_each(|(i, pixel)| {
            let (x, y) = ((i as u32) % width, (i as u32) / width);
            let (mut r_total, mut g_total, mut b_total, mut count) = (0, 0, 0, 0);

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        let p = img.get_pixel(nx as u32, ny as u32);
                        r_total += p[0] as u32;
                        g_total += p[1] as u32;
                        b_total += p[2] as u32;
                        count += 1;
                    }
                }
            }

            pixel.copy_from_slice(&[
                (r_total / count) as u8,
                (g_total / count) as u8,
                (b_total / count) as u8,
            ]);
        });

    *img = new_img;
}

pub fn apply_edge_detection(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let (width, height) = img.dimensions();
    let mut new_img = ImageBuffer::new(width, height);

    new_img
        .par_chunks_mut(3)
        .enumerate()
        .for_each(|(i, pixel)| {
            let (x, y) = ((i as u32) % width, (i as u32) / width);
            let mut gx = [0i32; 3];
            let mut gy = [0i32; 3];

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        let p = img.get_pixel(nx as u32, ny as u32);
                        let (weight_x, weight_y) = (dx, dy);
                        for c in 0..3 {
                            gx[c] += weight_x * p[c] as i32;
                            gy[c] += weight_y * p[c] as i32;
                        }
                    }
                }
            }

            for c in 0..3 {
                pixel[c] = ((gx[c].pow(2) + gy[c].pow(2)) as f32).sqrt().min(255.0) as u8;
            }
        });

    *img = new_img;
}

pub fn create_buffer(size: usize) -> Vec<u8> {
    vec![0; size]
}

pub fn buffer_to_image(buffer: &[u8], width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_raw(width, height, buffer.to_vec()).unwrap()
}

pub fn image_to_buffer(img: &ImageBuffer<Rgb<u8>, Vec<u8>>, buffer: &mut [u8]) {
    buffer.copy_from_slice(img.as_raw());
}

pub fn open_shared_memory(path: &Path, size: usize) -> Result<MmapMut> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    // Set the file size to match the required size
    file.set_len(size as u64)?;

    unsafe { MmapOptions::new().map_mut(&file) }.map_err(|e| eyre!("Failed to map file: {}", e))
}

pub fn shared_memory_to_image(
    mmap: &[u8],
    width: u32,
    height: u32,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let expected_len = (width * height * 3) as usize;
    if mmap.len() != expected_len {
        return Err(eyre!(
            "Mismatch in shared memory size. Expected {} bytes, got {} bytes",
            expected_len,
            mmap.len()
        ));
    }

    ImageBuffer::from_raw(width, height, mmap.to_vec()).ok_or_else(|| {
        eyre!(
            "Failed to create image from shared memory. Width: {}, Height: {}, Data length: {}",
            width,
            height,
            mmap.len()
        )
    })
}

pub fn image_to_shared_memory(
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    mmap: &mut MmapMut,
) -> Result<()> {
    let img_data = img.as_raw();
    if mmap.len() != img_data.len() {
        return Err(eyre!(
            "Mismatch in shared memory size. Expected {} bytes, got {} bytes",
            img_data.len(),
            mmap.len()
        ));
    }

    mmap.copy_from_slice(img_data);
    Ok(())
}

pub fn process_image<F>(config: &ProcessConfig, filter: F) -> Result<()>
where
    F: Fn(&mut ImageBuffer<Rgb<u8>, Vec<u8>>),
{
    if config.shared_memory.is_some() != config.width.is_some()
        || config.shared_memory.is_some() != config.height.is_some()
    {
        return Err(eyre!(
            "Shared memory, width, and height must all be provided together for coordinated mode"
        ));
    }

    if let Some(shared_memory_path) = &config.shared_memory {
        // Coordinated mode
        let (width, height) = (config.width.unwrap(), config.height.unwrap());
        let size = (width * height * 3) as usize;
        let mut mmap = open_shared_memory(shared_memory_path, size)?;
        let mut img = shared_memory_to_image(&mmap, width, height)?;
        filter(&mut img);
        image_to_shared_memory(&img, &mut mmap)?;
    } else {
        // Standalone mode
        let (input, output) = (
            config
                .input
                .as_ref()
                .ok_or_else(|| eyre!("Input file is required in standalone mode"))?,
            config
                .output
                .as_ref()
                .ok_or_else(|| eyre!("Output file is required in standalone mode"))?,
        );
        let mut img = load_image(input)?;
        filter(&mut img);
        save_image(&img, output)?;
    }

    Ok(())
}
