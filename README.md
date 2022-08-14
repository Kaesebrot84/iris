# Iris

[![Rust](https://github.com/Kaesebrot84/iris/actions/workflows/Build.yml/badge.svg)](https://github.com/Kaesebrot84/iris/actions/workflows/Build.yml)
[![crates.io](https://img.shields.io/crates/v/iris-lib.svg)](https://crates.io/crates/cliris)

A command line tool that creates color palettes from images using the [median
cut algorithm](https://en.wikipedia.org/wiki/Median_cut).

## Usage

```sh
$ cliris --help
cliris 0.2.0
Andrej G. <REDACTED@gmail.com>
A cli tool that creates color palettes from images using the median cut algorithm.

USAGE:
    cliris [OPTIONS] --file-name <FILE_NAME> [OUTPUT_FORMAT]

ARGS:
    <OUTPUT_FORMAT>    Desired data file format to be written [default: none] [possible values:
                       none, html, json, csv]

OPTIONS:
    -f, --file-name <FILE_NAME>          Target image file name
    -h, --help                           Print help information
    -i, --iterations <ITERATIONS>        Number of iterations [default: 1]
    -o, --out-filename <OUT_FILENAME>    File path the file should be written to [default: palette]
    -V, --version                        Print version information
```

### Installation

```bash
cargo install cliris
```

### Example

```sh
$ cliris -f peppers.png -i 3 html

Generating palette...
Finished generating palette in 75 ms.

{ R: 191, G: 207, B: 141, A: 255 }
{ R: 139, G: 187, B: 108, A: 255 }
{ R: 171, G: 185, B: 76, A: 255 }
{ R: 118, G: 159, B: 71, A: 255 }
{ R: 197, G: 60, B: 50, A: 255 }
{ R: 186, G: 41, B: 34, A: 255 }
{ R: 117, G: 77, B: 45, A: 255 }
{ R: 78, G: 7, B: 6, A: 255 }
```

<p align="center">
    <img src="example_output.png" alt="example_output_image" width="400">
</p>

### Library

This project uses the [iris-lib](https://crates.io/crates/iris-lib) crate, which performs this algorithm as a stand-alone library.
