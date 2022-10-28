use std::path::{Path, PathBuf};

use argh::FromArgs;
use crossterm::style::Stylize;
use image::{DynamicImage, ImageBuffer, ImageError, Rgb};
use vec2::Vec2;

mod vec2;

/// Turn any image into a mosaic!
#[derive(FromArgs, Debug)]
struct Command {
    /// the size of each mosaic tile
    #[argh(positional, from_str_fn(parse_vec2))]
    tile_size: Vec2<usize>,
    /// the template image
    #[argh(positional)]
    template: PathBuf,
    /// the images to grab tiles from
    #[argh(positional)]
    sources: Vec<PathBuf>,
    /// where to save the resulting image
    #[argh(option, short = 'o')]
    out_file: PathBuf,
}

fn main() {
    let Command {
        tile_size,
        template,
        sources,
        out_file,
    } = argh::from_env();

    if tile_size.into_iter().any(|v| v == 0) {
        println!("{}", "Tile size must be at least 1x1".red());
        return;
    }

    match load_image(&template) {
        Ok(template) => {
            let temp = 0;

            match template.save(out_file) {
                Ok(_) => println!("{}", "Successfully created mosaic!".green()),
                Err(e) => println!("{}", format!("Unable to save mosaic: {e}").red()),
            }
        }
        Err(e) => println!("{}", format!("Unable to load template: {e}").red()),
    }
}

fn parse_vec2(s: &str) -> Result<Vec2<usize>, String> {
    if let Some((x, y)) = s.split_once('x') {
        if let (Ok(x), Ok(y)) = (x.parse(), y.parse()) {
            Ok(Vec2::new(x, y))
        } else {
            Err("Expects inputs like \"4x4\"".to_owned())
        }
    } else {
        Err("Expects inputs like \"4x4\"".to_owned())
    }
}

fn load_image(path: &Path) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, ImageError> {
    image::io::Reader::open(path)?
        .decode()
        .map(DynamicImage::into_rgb8)
}
