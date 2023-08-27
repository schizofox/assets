
extern crate image;
extern crate structopt;

use std::fs;
use std::path::Path;
use image::{ImageError, imageops::FilterType};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input_image: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    output_directory: std::path::PathBuf,

    #[structopt(short = "s", long = "start-resolution")]
    start_resolution: String,

    #[structopt(short = "e", long = "end-resolution")]
    end_resolution: String,
}



fn generate_versions(input_path: &str, output_dir: &str, start_width: u32, start_height: u32, end_width: u32, end_height: u32) -> Result<(), ImageError> {
    let original_image = image::open(input_path)?;

    let png_output_dir = format!("{}/png", output_dir);

    fs::create_dir_all(&png_output_dir)?;

    let mut current_width = start_width;
    let mut current_height = start_height;

    while current_width <= end_width && current_height <= end_height {
        println!("Generating image for {}x{}", current_width, current_height);

        let resized_image = original_image.resize_exact(current_width, current_height, FilterType::Lanczos3);

        let png_output_path = format!("{}/resized_{}x{}_{}", png_output_dir, current_width, current_height, Path::new(input_path).file_name().unwrap().to_str().unwrap());
        resized_image.save(png_output_path.clone())?;
        println!("Generated PNG: {}", png_output_path);

        // Double the dimensions after each image is generated
        current_width *= 2;
        current_height *= 2;
    }

    Ok(())
}



fn main() -> Result<(), ImageError> {
    let opt = Opt::from_args();

    let start_dimensions: Vec<&str> = opt.start_resolution.split('x').collect();
    let end_dimensions: Vec<&str> = opt.end_resolution.split('x').collect();

    if start_dimensions.len() != 2 || end_dimensions.len() != 2 {
        eprintln!("Invalid start or end resolution format");
        return Ok(());
    }

    let start_width = start_dimensions[0].parse::<u32>().unwrap();
    let start_height = start_dimensions[1].parse::<u32>().unwrap();

    let end_width = end_dimensions[0].parse::<u32>().unwrap();
    let end_height = end_dimensions[1].parse::<u32>().unwrap();

    generate_versions(
        &opt.input_image.to_string_lossy(),
        &opt.output_directory.to_string_lossy(),
        start_width,
        start_height,
        end_width,
        end_height,
    )?;

    Ok(())
}

