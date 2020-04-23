use crate::op::ImageOperation;
use image::imageops::FilterType;
use image::DynamicImage;
use std::error::Error;

pub struct ScaleImage {
    width: u32,
    height: u32,
    filter: FilterType,
}
impl ScaleImage {
    pub fn new(width: u32, height: u32, filter: FilterType) -> Self {
        ScaleImage {
            width,
            height,
            filter,
        }
    }
}
impl ImageOperation for ScaleImage {
    fn execute(&mut self, image: &DynamicImage) -> Result<DynamicImage, Box<dyn Error>> {
        //let result = image.resize_to_fill(self.width, self.height, self.filter);
        let result = image.resize(self.width, self.height, self.filter);
        Ok(result)
    }
}
