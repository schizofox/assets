extern crate image;
use image::GenericImageView;

use std::env;
use std::fs;
use std::path::Path;
use image::{ImageError, imageops::FilterType};



fn generate_versions(input_path: &str, output_dir: &str, dimensions: &[(u32, u32)]) -> Result<(), ImageError> {
    let original_image = image::open(input_path)?;

    let (_original_width, _original_height) = original_image.dimensions();

    let png_output_dir = format!("{}/png", output_dir);

    fs::create_dir_all(&png_output_dir)?;

    for (width, height) in dimensions {
        let resized_image = original_image.resize_exact(*width, *height, FilterType::Lanczos3);

        // Clone the value of png_output_path
        let png_output_path = format!("{}/resized_{}x{}_{}", png_output_dir, width, height, Path::new(input_path).file_name().unwrap().to_str().unwrap());
        resized_image.save(png_output_path.clone())?; // Use the cloned value
        println!("Generated PNG: {}", png_output_path);
    }

    Ok(())
}

fn main() -> Result<(), ImageError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <input_image> <output_directory> <width1xheight1> <width2xheight2> ...", args[0]);
        return Ok(());
    }

    let input_image = &args[1];
    let output_directory = &args[2];

    let mut dimensions: Vec<(u32, u32)> = Vec::new();
    for i in 3..args.len() {
        let parts: Vec<&str> = args[i].split('x').collect();
        if parts.len() != 2 {
            eprintln!("Invalid dimensions: {}", args[i]);
            return Ok(());
        }
        let width = parts[0].parse::<u32>().unwrap();
        let height = parts[1].parse::<u32>().unwrap();
        dimensions.push((width, height));
    }

    generate_versions(input_image, output_directory, &dimensions)?;

    Ok(())
}
