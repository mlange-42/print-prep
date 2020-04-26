//! Prepare images for printing.

use crate::cli::parse;
use crate::op::{ImageIoOperation, ImageOperation};
use crate::units::color::Color;
use crate::units::{format, LengthUnit};
use crate::units::{Borders, Size};
use crate::util::ImageUtil;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

/// Prepare images for printing.
#[allow(dead_code)]
#[derive(StructOpt, Debug)]
pub struct PrepareImage {
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

    /// Image resolution. Default `300`.
    #[structopt(short, long)]
    pub dpi: Option<f64>,

    /// Filter type for image scaling.
    /// One of `(nearest|linear|cubic|gauss|lanczos)`.
    /// Default: `cubic`.
    #[structopt(short, long, parse(try_from_str = parse::parse_filter_type))]
    pub filter: Option<FilterType>,

    /// Print format `width/height`.
    /// Formats in cm are converted to exact print formats in inches.
    /// Examples: `15cm/10cm`, `6in/4in`, `6000px/4000px`.
    ///
    /// To use an exact size given in cm, use floating point numbers, e.g. `15.0cm/10.0cm`.
    #[structopt(long)]
    pub format: Size,

    /// Maximum image size, excl. padding.
    #[structopt(name = "image-size", long)]
    pub image_size: Option<Size>,

    /// Maximum image size, incl. padding.
    #[structopt(name = "framed-size", long)]
    pub framed_size: Option<Size>,

    /// Padding.
    #[structopt(long)]
    pub padding: Option<Borders>,

    /// Margins.
    #[structopt(long)]
    pub margins: Option<Borders>,

    /// Background color. Default `white`.
    #[structopt(short, long)]
    pub bg: Option<Color>,

    /// Prevents rotation of portrait format images (or of landscape format images if output is portrait).
    #[structopt(name = "no-rotation", long)]
    pub no_rotation: bool,
}
impl PrepareImage {
    fn check(&self) -> Result<(), Box<dyn Error>> {
        if self.format.width().is_none() || self.format.height().is_none() {
            return Err(Box::new(format::PrintFormatError(format!(
                "Missing dimension in print format {}",
                &self.format
            ))));
        }

        let mut count = 0;
        for v in [&self.image_size, &self.framed_size].iter() {
            if v.is_some() {
                count += 1;
            }
        }
        for v in [&self.padding, &self.margins].iter() {
            if v.is_some() {
                count += 1;
            }
        }

        if count != 2 {
            return Err(Box::new(format::PrintFormatError(format!(
                "Over- or under-determined print format. \
                Exactly two of the following options must be given: \
                `image-size`, `framed-size`, `padding`, `margins`. \
                The only invalid combination is `framed-size` and `margins`"
            ))));
        }

        if self.framed_size.is_some() && self.margins.is_some() {
            return Err(Box::new(format::PrintFormatError(format!(
                "Invalid combination of print format options. \
                Exactly two of the following options must be given: \
                `image-size`, `framed-size`, `padding`, `margins`. \
                The only invalid combination is `framed-size` and `margins`"
            ))));
        }

        Ok(())
    }
}

impl ImageOperation for PrepareImage {
    fn execute(&self, files: &[PathBuf]) -> Result<(), Box<dyn Error>> {
        ImageIoOperation::execute(self, &files)
    }
}

impl ImageIoOperation for PrepareImage {
    fn output(&self) -> &str {
        &self.output
    }

    fn quality(&self) -> &Option<u8> {
        &self.quality
    }

    fn process_image(&self, image: &DynamicImage) -> Result<DynamicImage, Box<dyn Error>> {
        self.check()?;

        let dpi = self.dpi.unwrap_or(300.0);
        let filter = self.filter.as_ref().unwrap_or(&FilterType::CatmullRom);
        let color = self.bg.clone().unwrap_or(Color::new(255, 255, 255, 255));
        let format = format::to_print_format(&self.format)?.to(&LengthUnit::Px, dpi);

        let width = format.width().as_ref().unwrap().value().round() as u32;
        let height = format.height().as_ref().unwrap().value().round() as u32;
        let in_is_portrait = image.height() > image.width();
        let out_is_portrait = height > width;

        let (width, height) = if self.no_rotation || in_is_portrait == out_is_portrait {
            (width, height)
        } else {
            (height, width)
        };

        let mut result = if image.color().has_alpha() {
            DynamicImage::new_rgba8(width, height)
        } else {
            DynamicImage::new_rgb8(width, height)
        };
        ImageUtil::fill_image(&mut result, color.channels());

        Ok(result)
    }
}
