//! Image utilities

use crate::util::PathUtil;
use image::flat::SampleLayout;
use image::DynamicImage;
use path_absolutize::Absolutize;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;

/// Image utilities
pub struct ImageUtil {}

impl ImageUtil {
    /// Saves an image to a file
    pub fn save_image(
        image: DynamicImage,
        out_path: &PathBuf,
        quality: u8,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(samples) = image.as_flat_samples_u8() {
            Self::save_buffer(samples.samples, &samples.layout, out_path, quality)?;
            Ok(())
        } else {
            // TODO: implement for 16 bit images
            Err(Box::new(ImageFormatError(
                "This is not an 8-bit per channel image.".to_string(),
            )))
        }
    }

    /// Saves an image buffer to a file
    pub fn save_buffer(
        buffer: &[u8],
        layout: &SampleLayout,
        out_path: &PathBuf,
        quality: u8,
    ) -> Result<(), Box<dyn Error>> {
        let abs_path = out_path.absolutize()?;
        let ext = Self::prepare_save(&abs_path)?;

        if ext == "jpg" || ext == "jpeg" {
            let mut file = File::create(&abs_path)?;
            let mut enc = image::jpeg::JPEGEncoder::new_with_quality(&mut file, quality);
            enc.encode(
                &buffer,
                layout.width,
                layout.height,
                if layout.width_stride == 4 {
                    image::ColorType::Rgba8
                } else {
                    image::ColorType::Rgb8
                },
            )
            .expect(&format!("Unable to write output file {:?}.", &abs_path));
        } else {
            image::save_buffer(
                &abs_path,
                &buffer,
                layout.width,
                layout.height,
                if layout.width_stride == 4 {
                    image::ColorType::Rgba8
                } else {
                    image::ColorType::Rgb8
                },
            )
            .expect(&format!("Unable to save output file {:?}", &abs_path));
        }

        Ok(())
    }

    fn prepare_save(path: &PathBuf) -> Result<String, Box<dyn Error>> {
        let ext = PathUtil::extension(&path).ok_or(InvalidImagePathError(
            "Expects an extension for output file to determine image format.".to_string(),
        ))?;

        let parent = path.parent().ok_or(InvalidImagePathError(format!(
            "Not a valid output path: {:?}",
            path
        )))?;

        if !parent.is_dir() {
            std::fs::create_dir(parent)?;
        }

        Ok(ext)
    }
}

/// Error type for invalid image path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidImagePathError(String);

impl Error for InvalidImagePathError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl fmt::Display for InvalidImagePathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Error type for invalid image format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageFormatError(pub String);

impl Error for ImageFormatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
impl fmt::Display for ImageFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
