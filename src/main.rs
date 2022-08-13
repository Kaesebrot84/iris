extern crate iris_lib;

use iris_lib::color_bucket::ColorBucket;
use std::time::Instant;

use crate::export::*;

use clap::{ArgEnum, Parser};

pub mod export;

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

    let now = Instant::now();
    println!("Generating palette...");

    if let Some(mut color_bucket) = ColorBucket::from_image(&args.file_name) {
        let palette = color_bucket.make_palette(num_iterations);
        println!("Finished generating palette in {} ms.\n", now.elapsed().as_millis());

        for color in &palette {
            println!("{}", color);
        }

        match args.output_format {
            OutputFormat::Html => match write_html_out(&args.file_name, &palette, &args.out_filename) {
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
    }
}
