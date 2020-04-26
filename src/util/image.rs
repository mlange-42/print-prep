//! Image utilities

use crate::units::color::Color;
use crate::units::ScaleMode;
use crate::util::PathUtil;
use image::flat::SampleLayout;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use path_absolutize::Absolutize;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;

/// Image utilities
pub struct ImageUtil {}

impl ImageUtil {
    pub fn fill_image(image: &mut DynamicImage, color: &[u8; 4]) {
        let col = Rgba(*color);
        for y in 0..image.height() {
            for x in 0..image.width() {
                image.put_pixel(x, y, col);
            }
        }
    }
    pub fn scale_image(
        image: &DynamicImage,
        width: u32,
        height: u32,
        mode: &ScaleMode,
        filter: &FilterType,
        background: &Color,
        incremental: bool,
    ) -> Result<DynamicImage, Box<dyn Error>> {
        if incremental && image.width() > 3 * width && image.height() > 3 * height {
            let mut img = Self::scale_to_half(image)?;
            while img.width() > 3 * width && img.height() > 3 * height {
                img = Self::scale_to_half(&img)?;
            }
            Self::scale_image_simple(&img, width, height, mode, filter, background)
        } else {
            Self::scale_image_simple(image, width, height, mode, filter, background)
        }
    }

    pub fn scale_image_simple(
        image: &DynamicImage,
        width: u32,
        height: u32,
        mode: &ScaleMode,
        filter: &FilterType,
        background: &Color,
    ) -> Result<DynamicImage, Box<dyn Error>> {
        let result = match mode {
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
                Self::fill_image(&mut result, background.channels());

                let x = (result.width() - temp.width()) / 2;
                let y = (result.height() - temp.height()) / 2;
                result.copy_from(&temp, x, y)?;
                result
            }
        };
        Ok(result)
    }

    fn scale_to_half(image: &DynamicImage) -> Result<DynamicImage, Box<dyn Error>> {
        let width = image.width() / 2;
        let height = image.height() / 2;

        let (mut result, channels) = if image.color().has_alpha() {
            (DynamicImage::new_rgba8(width, height), 4)
        } else {
            (DynamicImage::new_rgb8(width, height), 3)
        };

        let mut col = Rgba([0, 0, 0, 255]);
        let mut mean: [u16; 4] = [0, 0, 0, 255];
        for y in 0..result.height() {
            for x in 0..result.width() {
                for c in 0..channels {
                    mean[c] = 0;
                }
                for yy in (y * 2)..(y * 2 + 2) {
                    for xx in (x * 2)..(x * 2 + 2) {
                        let pix = image.get_pixel(xx, yy);
                        for c in 0..channels {
                            mean[c] += pix.0[c] as u16;
                        }
                    }
                }
                for c in 0..channels {
                    col[c] = (mean[c] as f32 / 4.0).round() as u8;
                }
                result.put_pixel(x, y, col);
            }
        }

        Ok(result)
    }

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

#[cfg(test)]
mod test {
    use crate::units::color::Color;
    use crate::units::ScaleMode;
    use crate::util::ImageUtil;
    use image::imageops::FilterType;
    use image::{DynamicImage, GenericImageView};

    #[test]
    fn fill_image() {
        let mut image = DynamicImage::new_rgb8(32, 32);
        let col = [255, 0, 0, 255];
        ImageUtil::fill_image(&mut image, &col);

        assert_eq!(image.get_pixel(0, 0).0, col);
    }

    #[test]
    fn scale_image() {
        let image = DynamicImage::new_rgb8(256, 256);
        let scaled = ImageUtil::scale_image(
            &image,
            32,
            32,
            &ScaleMode::Keep,
            &FilterType::CatmullRom,
            &Color::new(255, 255, 255, 255),
            false,
        )
        .unwrap();

        assert_eq!(scaled.width(), 32);
        assert_eq!(scaled.height(), 32);
    }

    #[test]
    fn scale_image_inc() {
        let image = DynamicImage::new_rgb8(256, 256);
        let scaled = ImageUtil::scale_image(
            &image,
            32,
            32,
            &ScaleMode::Keep,
            &FilterType::CatmullRom,
            &Color::new(255, 255, 255, 255),
            true,
        )
        .unwrap();

        assert_eq!(scaled.width(), 32);
        assert_eq!(scaled.height(), 32);
    }

    #[test]
    fn scale_to_half() {
        let image = DynamicImage::new_rgb8(64, 64);
        let scaled = ImageUtil::scale_to_half(&image).unwrap();

        assert_eq!(scaled.width(), 32);
        assert_eq!(scaled.height(), 32);
    }
}
