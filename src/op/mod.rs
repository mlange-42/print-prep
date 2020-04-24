//! `print-prep` operations
use std::error::Error;

pub mod scale;

use crate::util::image::ImageFormatError;
use crate::util::{ImageUtil, PathUtil};
use image::DynamicImage;
use indicatif::ProgressBar;
use rayon::prelude::*;
pub use scale::ScaleImage;
use std::path::PathBuf;

/// Trait for all image operations.
pub trait ImageOperation {
    fn execute(&self, files: &[PathBuf]) -> Result<(), Box<dyn Error>>;
}

/// Trait for all image operations.
pub trait ImageIoOperation: ImageOperation + Send + Sync {
    fn output(&self) -> &str;
    fn quality(&self) -> &Option<u8>;
    fn process_image(&self, image: &DynamicImage) -> Result<DynamicImage, Box<dyn Error>>;
    fn execute(&self, files: &[PathBuf]) -> Result<(), Box<dyn Error>> {
        let bar = ProgressBar::new(files.len() as u64);
        files
            .par_iter()
            .map(|file: &PathBuf| {
                bar.inc(1);

                let out_path = match PathUtil::out_path(file, &self.output()) {
                    Some(p) => p,
                    None => {
                        return Err(ImageFormatError(format!(
                            "Unable to generate output file name from {:?}",
                            self.output()
                        )));
                    }
                };

                let input = match image::open(file) {
                    Ok(i) => i,
                    Err(e) => {
                        return Err(ImageFormatError(format!(
                            "Unable to read image {:?} ({:?})",
                            file, e
                        )));
                    }
                };

                let output = match self.process_image(&input) {
                    Ok(o) => o,
                    Err(e) => {
                        return Err(ImageFormatError(format!(
                            "Unable to process image {:?}: {:?}",
                            file,
                            e.to_string()
                        )));
                    }
                };

                match ImageUtil::save_image(output, &out_path, self.quality().unwrap_or(95)) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(ImageFormatError(format!(
                            "Unable to save image to {:?}: {:?}",
                            out_path,
                            e.to_string()
                        )));
                    }
                };

                Ok(())
            })
            .collect::<Result<(), ImageFormatError>>()?;
        bar.finish_and_clear();
        Ok(())
    }
}
