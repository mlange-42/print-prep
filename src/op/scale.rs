use crate::cli::parse;
use crate::op::{ImageIoOperation, ImageOperation};
use crate::units::color::RGBA;
use crate::units::length::{Length, LengthUnit, Scale, ScaleMode, Size};
use crate::OperationParametersError;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

/// Scale images.
#[derive(StructOpt, Debug)]
pub struct ScaleImage {
    /// Output path. Use `*` as placeholder for the original base file name.
    /// Used to determine output image type.
    ///
    /// Examples:
    /// --output "path/to/*-out.jpg"
    ///
    #[structopt(verbatim_doc_comment)]
    #[structopt(short, long)]
    pub output: String,

    /// Image quality for JPEG output in percent. Optional, default `95`.
    #[structopt(short, long)]
    pub quality: Option<u8>,

    /// Output image size. Use either `--size` or `--scale`.
    /// Examples: `100px/.`, `./15cm`, `8in/6in`.
    #[structopt(long)]
    size: Option<Size>,

    /// Output image scale. Use either `--size` or `--scale`.
    /// Examples: `0.5`, `50%`, `20%/10%`.
    #[structopt(long)]
    scale: Option<Scale>,

    /// Scaling mode. Must be given when using `--size` with width and height.
    /// One of `(keep|stretch|crop|fill)`.
    /// Default: `keep`.
    #[structopt(long)]
    mode: Option<ScaleMode>,

    /// Filter type for image scaling.
    /// One of `(nearest|linear|cubic|gauss|lanczos)`.
    /// Default: `cubic`.
    #[structopt(long, parse(try_from_str = parse::parse_filter_type))]
    filter: Option<FilterType>,

    /// Image resolution for size not in px. Default `300`.
    #[structopt(long)]
    dpi: Option<f32>,

    /// Background color for `--mode fill`. Default `white`.
    #[structopt(long)]
    bg: Option<RGBA>,
}
impl ScaleImage {
    fn check(&self) -> Result<(), Box<dyn Error>> {
        if self.size.is_some() == self.scale.is_some() {
            return Err(Box::new(OperationParametersError(
                "Exactly one of `--size` and `--scale` must be given!".to_string(),
            )));
        }
        Ok(())
    }
}

impl ImageOperation for ScaleImage {
    fn execute(&self, files: &[PathBuf]) -> Result<(), Box<dyn Error>> {
        ImageIoOperation::execute(self, &files)
    }
}

impl ImageIoOperation for ScaleImage {
    fn output(&self) -> &str {
        &self.output
    }

    fn quality(&self) -> &Option<u8> {
        &self.quality
    }

    fn process_image(&self, image: &DynamicImage) -> Result<DynamicImage, Box<dyn Error>> {
        self.check()?;

        let dpi = self.dpi.unwrap_or(300.0);
        let size = if let Some(s) = &self.size {
            s.to(&LengthUnit::Px, dpi)
        } else {
            Size::new(
                Some(Length::px(
                    (image.width() as f32 * self.scale.as_ref().unwrap().width()).round() as i32,
                )),
                Some(Length::px(
                    (image.height() as f32 * self.scale.as_ref().unwrap().height()).round() as i32,
                )),
            )?
        };
        let filter = self.filter.as_ref().unwrap_or(&FilterType::CatmullRom);
        let mode = self.mode.as_ref().unwrap_or(&ScaleMode::Keep);
        let color = self.bg.clone().unwrap_or(RGBA::new(255, 255, 255, 255));

        let mut any_missing = false;
        let width = if let Some(w) = size.width() {
            w.value() as u32
        } else {
            any_missing = true;
            let h = size.height().as_ref().unwrap().value();
            ((h as f64 / image.height() as f64) * image.width() as f64).round() as u32
        };
        let height = if let Some(h) = size.height() {
            h.value() as u32
        } else {
            any_missing = true;
            let w = size.width().as_ref().unwrap().value();
            ((w as f64 / image.width() as f64) * image.height() as f64).round() as u32
        };

        let result = if any_missing {
            image.resize(width, height, *filter)
        } else {
            match mode {
                ScaleMode::Keep => image.resize(width, height, *filter),
                ScaleMode::Stretch => image.resize_exact(width, height, *filter),
                ScaleMode::Crop => image.resize_to_fill(width, height, *filter),
                ScaleMode::Fill => {
                    let temp = image.resize(width, height, *filter);
                    let mut result = if temp.color().has_alpha() {
                        DynamicImage::new_rgba8(width, height)
                    } else {
                        DynamicImage::new_rgb8(width, height)
                    };
                    let col = Rgba(*color.channels());
                    for y in 0..result.height() {
                        for x in 0..result.width() {
                            result.put_pixel(x, y, col);
                        }
                    }
                    let x = (result.width() - temp.width()) / 2;
                    let y = (result.height() - temp.height()) / 2;
                    result.copy_from(&temp, x, y)?;
                    result
                }
            }
        };
        Ok(result)
    }
}
