//! `print-prep` operations
use image::DynamicImage;
use std::error::Error;

trait ImageOperation {
    fn execute(&mut self, image: &DynamicImage) -> Result<DynamicImage, Box<dyn Error>>;
}
