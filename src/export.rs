use std::fs::File;
use std::io::Write;

use iris_lib::color::Color;

/// Writes a html file containing the target image and the according colors.
///
/// TODO: Refactor
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
    let mut html: String = String::new();

    html.push_str("<!DOCTYPE html><head>");
    html.push_str("<meta content=\"width=device-width, initial-scale=1\" http-equiv=\"Content-Type\">");
    html.push_str("<meta content=\"utf-8\" http-equiv=\"encoding\">");
    html.push_str("<style> img {display: block; margin-left: auto; margin-right: auto; }</style>");
    html.push_str("</head>");
    html.push_str("<html><body>");
    let image_element = format!("<img src=\"{}\" alt=\"Input image\" class=\"centered\">", image_file_path);
    html.push_str(image_element.as_str());
    html.push_str("</b>");
    html.push_str("<div style=\"display: grid;align-items: center;justify-content: center;gap: 5px;width: 100%;padding-top: 10px;grid-auto-flow: column;\">");

    for c in color_data {
        let color_element = format!(
            "<div style=\"background-color:rgb({},{},{});display: flex;justify-content: center;align-items: center;height: 100px;width: 100px;\"></div>",
            c.r, c.g, c.b
        );
        html.push_str(color_element.as_str());
    }

    html.push_str("</div>");
    html.push_str("</body></html>");

    let mut file = File::create(format!("{}.html", out_file_path))?;
    file.write_all(html.as_bytes())?;
    Ok(())
}

/// Writes a color palette to a json file.
///
/// # Arguments
///
/// * `color_data` - Colors to be written as a palette to json.
/// * `out_file_path` - Path the html file should be written to.
///
/// # Examples
///
/// ```
/// let colors = vec![Color {r: 255, g: 0, b: 0, a: 255}];
/// write_json_out(&colors, "palette")?;
/// ```
///
pub fn write_json_out(color_data: &[Color], out_file_path: &str) -> std::io::Result<()> {
    let mut json: String = String::new();
    json.push('{');
    json.push_str("\"palette\": [");

    let mut color_it = color_data.iter().peekable();

    while let Some(color) = color_it.next() {
        json.push_str(format!("{{ \"r\": {}, \"g\": {}, \"b\": {}, \"a\": {} }}", color.r, color.g, color.b, color.a).as_str());
        if color_it.peek().is_some() {
            json.push(',');
        }
    }
    json.push_str("]}");

    let mut file = File::create(format!("{}.json", out_file_path))?;
    file.write_all(json.as_bytes())?;

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
    let mut csv: String = String::new();

    csv.push_str("R, G, B, A\n");

    for color in color_data {
        csv.push_str(format!("{}, {}, {}, {}\n", color.r, color.g, color.b, color.a).as_str());
    }

    let mut file = File::create(format!("{}.csv", out_file_path))?;
    file.write_all(csv.as_bytes())?;
    Ok(())
}
