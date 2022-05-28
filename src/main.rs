extern crate image;

use std::time::Instant;

use crate::color::*;
use crate::color_bucket::ColorBucket;
use crate::export::*;

use clap::{ArgEnum, Parser};
use image::GenericImageView;

pub mod color;
pub mod color_bucket;
pub mod export;
pub mod utils;

/// A command line tool that creates color palettes from images using the median cut algorithm.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Target image file name.
    #[clap(short, long)]
    file_name: String,

    /// Number of iterations.
    #[clap(short, long, default_value_t = 1)]
    iterations: u8,

    /// Desired data file format to be written.
    #[clap(arg_enum, default_value_t = OutputFormat::None)]
    output_format: OutputFormat,

    /// File path the file should be written to.
    #[clap(short, long, default_value_t = String::from("palette"))]
    out_filename: String,
}

/// Represents all possible file output formats for color palettes.
#[derive(ArgEnum, Clone, Debug)]
enum OutputFormat {
    None,
    Html,
    Json,
    Csv,
}

fn main() {
    let args = Args::parse();

    let image_file_path = args.file_name;
    let num_iterations = match args.iterations {
        a if a > 4 => {
            println!("Switching to maximum number of iterations of 4.");
            4
        }
        a if a < 1 => {
            println!("Switching to minimum number of iterations");
            1
        }
        a => a,
    };

    if let Ok(img) = image::open(image_file_path.clone()) {
        println!("\nImage: {:?} - {:?}", img.dimensions(), img.color());

        let now = Instant::now();
        let mut pixels = Vec::new();

        for p in img.pixels() {
            let color = Color {
                r: p.2 .0[0],
                g: p.2 .0[1],
                b: p.2 .0[2],
                a: p.2 .0[3],
            };

            pixels.push(color);
        }
        println!("Finished reading {} pixel values in {} ms.", pixels.len(), now.elapsed().as_millis());

        let now = Instant::now();
        println!("Generating palette...");

        if let Some(mut color_bucket) = ColorBucket::from_pixels(pixels) {
            let palette = color_bucket.make_palette(num_iterations);
            println!("Finished generating palette in {} ms.\n", now.elapsed().as_millis());

            for color in &palette {
                println!("{}", color);
            }

            match args.output_format {
                OutputFormat::Html => match write_html_out(&image_file_path, &palette, &args.out_filename) {
                    Ok(_) => (),
                    Err(err) => println!("Failed writing html output file:\n{}", err),
                },
                OutputFormat::Json => match write_json_out(&palette, &args.out_filename) {
                    Ok(_) => (),
                    Err(err) => println!("Failed writing json output file:\n{}", err),
                },
                OutputFormat::Csv => match write_csv_out(&palette, &args.out_filename) {
                    Ok(_) => (),
                    Err(err) => println!("Failed writing csv output file:\n{}", err),
                },
                OutputFormat::None => (),
            }
        } else {
            println!("Failed generating color data from the image.");
        }
    } else {
        println!("Unable to locate file: {}", image_file_path);
    }
}
