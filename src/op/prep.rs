//! Prepare images for printing.

use crate::cli::parse;
use crate::op::{ImageIoOperation, ImageOperation};
use crate::units::color::Color;
use image::imageops::FilterType;
use image::DynamicImage;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

/// Prepare images for printing.
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
    pub dpi: Option<f32>,

    /// Filter type for image scaling.
    /// One of `(nearest|linear|cubic|gauss|lanczos)`.
    /// Default: `cubic`.
    #[structopt(short, long, parse(try_from_str = parse::parse_filter_type))]
    filter: Option<FilterType>,

    /// Background color. Default `white`.
    #[structopt(short, long)]
    bg: Option<Color>,
}
impl PrepareImage {
    fn check(&self) -> Result<(), Box<dyn Error>> {
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

        //Ok(result)
        unimplemented!()
    }
}
