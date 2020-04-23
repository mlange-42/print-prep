use crate::cli::parse;
use crate::op::ImageOperation;
use crate::units::length::{Length, LengthUnit, Scale, ScaleMode, Size};
use crate::OperationParametersError;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, GenericImageView};
use std::error::Error;
use structopt::StructOpt;

/// Scale images.
#[derive(StructOpt, Debug)]
pub struct ScaleImage {
    /// Output image size. Use either `--size` or `--scale`.
    #[structopt(long)]
    size: Option<Size>,
    /// Output image scale. Use either `--size` or `--scale`.
    #[structopt(long)]
    scale: Option<Scale>,
    /// Scaling mode. Must be given when using `--size` with width and height.
    #[structopt(long)]
    mode: Option<ScaleMode>,
    /// Filter type for image scaling
    #[structopt(long, parse(try_from_str = parse::parse_filter_type))]
    filter: Option<FilterType>,
    /// Image resolution for size not in px. Default `300`.
    #[structopt(long)]
    dpi: Option<f32>,
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
    fn execute(&self, image: &DynamicImage) -> Result<DynamicImage, Box<dyn Error>> {
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
