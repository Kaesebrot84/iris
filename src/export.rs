use std::fs::File;
use std::io::Write;

use crate::color::*;

/// Writes a html file containing the target image and the according colors.
///
/// TODO: Refactor
/// - Refactor the output html to be more elegant and responsive
/// - If output file is not in the same folder as image, image is not displayed. Write entire image path intp html instead.
///
/// # Arguments
///
/// * `image_file_path` - Image path to be included as `src` in the `html` file.
/// * `color_data` - Colors to be included as "tiles" in the html file.
/// * `out_file_path` - Path the output file should be written to.
///
/// # Examples
///
/// ```
/// let colors = vec![Color {r: 255, g: 0, b: 0, a: 255}];
/// write_html_out("example.jpg", &colors, "palette")?;
/// ```
///
pub fn write_html_out(image_file_path: &str, color_data: &[Color], out_file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.html", out_file_path))?;
    file.write_all(b"<!DOCTYPE html><head>")?;
    file.write_all(b"<meta content=\"width=device-width, initial-scale=1\" http-equiv=\"Content-Type\">")?;
    file.write_all(b"<meta content=\"utf-8\" http-equiv=\"encoding\">")?;
    file.write_all(b"<style> img {display: block; margin-left: auto; margin-right: auto; }</style>")?;
    file.write_all(b"</head>")?;
    file.write_all(b"<html><body>")?;
    let image_element = format!("<img src=\"{}\" alt=\"Input image\" class=\"centered\">", image_file_path);
    file.write_all(image_element.as_bytes())?;

    file.write_all(b"</b>")?;

    file.write_all(b"<div style=\"display: grid;align-items: center;justify-content: center;gap: 5px;width: 100%;padding-top: 10px;grid-auto-flow: column;\">")?;

    for c in color_data {
        let color_element = format!(
            "<div style=\"background-color:rgb({},{},{});display: flex;justify-content: center;align-items: center;height: 100px;width: 100px;\"></div>",
            c.r, c.g, c.b
        );
        file.write_all(color_element.as_bytes())?;
    }

    file.write_all(b"</div>")?;

    file.write_all(b"</body></html>")?;
    Ok(())
}

/// Writes a color palette to a json file.
///
/// # Arguments
///
/// * `color_data` - Colors to be written as a palette to json.
/// * `out_file_path` - Path the output file should be written to.
///
/// # Examples
///
/// ```
/// let colors = vec![Color {r: 255, g: 0, b: 0, a: 255}];
/// write_json_out(&colors, "palette")?;
/// ```
///
pub fn write_json_out(color_data: &[Color], out_file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.json", out_file_path))?;
    file.write_all(b"{")?;
    file.write_all(b"\"palette\": [")?;

    let mut color_it = color_data.iter().peekable();

    while let Some(color) = color_it.next() {
        file.write_all(format!("{{ \"r\": {}, \"g\": {}, \"b\": {}, \"a\": {} }}", color.r, color.g, color.b, color.a).as_bytes())?;
        if color_it.peek().is_some() {
            file.write_all(b",")?;
        }
    }

    file.write_all(b"]}")?;
    Ok(())
}

/// Writes a color palette to a csv file.
/// First row in the file will be headers R, G, B, A.
///
/// # Arguments
///
/// * `color_data` - Colors to be written to the csv file.
/// * `out_file_path` - Path the output file should be written to.
///
/// # Examples
///
/// ```
/// let colors = vec![Color {r: 255, g: 0, b: 0, a: 255}];
/// write_csv_out(&colors, "palette")?;
/// ```
///
pub fn write_csv_out(color_data: &[Color], out_file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.csv", out_file_path))?;
    file.write_all(b"R, G, B, A\n")?;

    for color in color_data {
        file.write_all(format!("{}, {}, {}, {}\n", color.r, color.g, color.b, color.a).as_bytes())?;
    }

    Ok(())
}
