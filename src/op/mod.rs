//! `print-prep` operations
use image::DynamicImage;
use std::error::Error;

pub mod scale;

pub use scale::ScaleImage;

pub trait ImageOperation {
    fn execute(&self, image: &DynamicImage) -> Result<DynamicImage, Box<dyn Error>>;
}
