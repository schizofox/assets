extern crate image;

use std::str::FromStr;
use std::{fs, fmt::Display};
use std::path::Path;
use image::{ImageError, imageops::FilterType};
use clap::Parser;

#[derive(Debug)]
enum DimensionType {
    Width,
    Height,
}

#[derive(thiserror::Error, Debug)]
enum DimParseError {
    InvalidDimensionCount,
    ParseError(DimensionType, String),
}

impl Display for DimParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::InvalidDimensionCount => "invalid dimension count".to_string(),
            Self::ParseError(r#type, message) => format!("failed parsing {:?}: {}", r#type, message),
        })
    }
}

#[derive(Clone)]
struct Dimension {
    width: u32,
    height: u32,
}

impl FromStr for Dimension {
    type Err = DimParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.trim().split("x").collect();

        use DimensionType::*;
        use DimParseError::*;

        if split.len() != 2 {
            return Err(InvalidDimensionCount);
        }

        let width = split[0];
        let height = split[1];

        let width = match width.parse::<u32>() {
            Ok(val) => val,
            Err(e) => {
                return Err(ParseError(Width, e.to_string()));
            },
        };

        let height = match height.parse::<u32>() {
            Ok(val) => val,
            Err(e) => {
                return Err(ParseError(Height, e.to_string()));
            },
        };

        Ok(Dimension { width, height })
    }
}

#[derive(Parser)]
struct Opt {
    #[arg(short = 'i', long, help = "TODO")]
    input_image: String,
    #[arg(short = 'o', long, help = "TODO")]
    output_dir: String,
    #[arg(short = 's', long, help = "TODO")]
    start_dim: Dimension,
    #[arg(short = 'e', long, help = "TODO")]
    end_dim: Dimension,
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
    let opt = Opt::parse();

    generate_versions(
        &opt.input_image,
        &opt.output_dir,
        opt.start_dim.width,
        opt.start_dim.height,
        opt.end_dim.width,
        opt.end_dim.height,
    )?;

    Ok(())
}

