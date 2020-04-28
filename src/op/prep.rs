//! Prepare images for printing.

use crate::cli::parse;
use crate::op::{ImageIoOperation, ImageOperation};
use crate::units::color::Color;
use crate::units::{format, FreeSize, Length, LengthUnit, ScaleMode};
use crate::units::{Borders, FixSize};
use crate::util::ImageUtil;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use imageproc::rect::Rect;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

/// Prepare images for printing (add cut marks, 'mats', test patterns, EXIF information, ...).
///
/// <pre>
///      ________________________________________
///     |    |                              |    |
///     |    |                              |    |-----  format
///     |---- ------------------------------ ----|
///     |    |     ____________________     |----|-----  framed-size
///     |    |    |                    |    |    |
///     |    |    |                    |----|----|-----  image-size
///     |    |    |                    |    |    |       border
///     |    |    |                    |    |    |
///     |    |    |                    |   -|----|-----  padding
///     |    |    |                    |    |    |
///     |    |    |                    |    |   -|-----  margins
///     |    |    |____________________|    |    |
///     |    |                              |----|-----  cut-frame
///     |---- ------------------------------ ----|
///     |    |                              |----|-----  cut-marks
///     |____|______________________________|____|
/// </pre>
#[doc(test(ignore))]
#[structopt(verbatim_doc_comment)]
#[allow(dead_code)]
#[derive(StructOpt, Debug)]
pub struct PrepareImage {
    /// Output path. Use `*` as placeholder for the original base file name.
    /// Used to determine output image type. On Unix systems, this MUST be quoted!
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

    /// Cut marks with offset. Format <line-width>/<offset>. Use alternative to `--cut-frame`.
    #[structopt(name = "cut-marks", long, value_name = "w/off")]
    pub cut_marks: Option<FreeSize>,

    /// Cut frame. Format <line-width>/<extend>. Use alternative to `--cut-marks`.
    #[structopt(name = "cut-frame", long, value_name = "w/off")]
    pub cut_frame: Option<FreeSize>,

    /// Cut marks, frame and exif color. Default: black.
    #[structopt(long, value_name = "color")]
    pub color: Option<Color>,

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
    #[structopt(long, value_name = "w/h")]
    pub format: FixSize,

    /// Maximum image size, excl. padding.
    #[structopt(name = "image-size", long, value_name = "w/h")]
    pub image_size: Option<FixSize>,

    /// Maximum image size, incl. padding.
    #[structopt(name = "framed-size", long, value_name = "w/h")]
    pub framed_size: Option<FixSize>,

    /// Padding between image and cut marks.
    #[structopt(long, value_name = "tp/rt/bm/lt")]
    pub padding: Option<Borders>,

    /// Minimum margins around cut marks.
    #[structopt(long, value_name = "tp/rt/bm/lt")]
    pub margins: Option<Borders>,

    /// Border width around image. Default none.
    /// This is included in padding!
    #[structopt(long, value_name = "tp/rt/bm/lt")]
    pub border: Option<Borders>,

    /// Border color. Default black.
    #[structopt(name = "border-color", long, value_name = "color")]
    pub border_color: Option<Color>,

    /// Enable incremental scaling.
    /// For scaling to small sizes, scales down in multiple steps, to 50% per step, averaging over 2x2 pixels.
    #[structopt(long)]
    pub incremental: bool,

    /// Background color. Default `white`.
    #[structopt(short, long, value_name = "color")]
    pub bg: Option<Color>,

    /// Prevents rotation of portrait format images
    /// (or of landscape format images if `--format` is portrait).
    #[structopt(name = "no-rotation", long)]
    pub no_rotation: bool,

    /// Prints exif data. Formatting string.
    /// Example: --exif "{F/2}, {Exp}, ISO {ISO}, {F}"
    /// Common abbreviations:
    /// `F/2`, `Exp`, `ISO`, `F`, `Bias`, `Date`, `Mod`.
    /// Further, all official exif tags.
    #[structopt(long, value_name = "format")]
    pub exif: Option<String>,

    /// Size of exif font, in arbitrary units. Default: `12px`.
    #[structopt(name = "exif-size", long, value_name = "size")]
    pub exif_size: Option<Length>,

    /// Prints a print control element, with the given square size and gap.
    /// Format: `<sx>/<gx>/<sy>/<gy>` or `<size>/<gap>`.
    /// Example: `10px/2px/10px/2px`
    #[structopt(name = "test-pattern", long, value_name = "sx/gx/sy/gy")]
    pub test_pattern: Option<Borders>,

    #[structopt(skip)]
    fonts: crate::Fonts,
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

    fn process_image(
        &self,
        image: &DynamicImage,
        file: &PathBuf,
    ) -> Result<DynamicImage, Box<dyn Error>> {
        self.check()?;

        let dpi = self.dpi.unwrap_or(300.0);
        let filter = self.filter.as_ref().unwrap_or(&FilterType::CatmullRom);
        let color = self.bg.clone().unwrap_or(Color::new(255, 255, 255, 255));
        let format = format::to_print_format(&self.format)?.to(&LengthUnit::Px, dpi);

        let width = format.width().value().round() as u32;
        let height = format.height().value().round() as u32;

        let in_is_portrait = image.height() > image.width();
        let out_is_portrait = height > width;
        let rotate = !(self.no_rotation || in_is_portrait == out_is_portrait);

        let (width, height) = if rotate {
            (height, width)
        } else {
            (width, height)
        };

        // Calculates sizes, etc.
        let (img, _frame, padding, margins) =
            self.calc_sizes(width, height, image.width(), image.height(), rotate, dpi);
        let x_img = (margins.left().value() + padding.left().value()) as u32;
        let y_img = (margins.top().value() + padding.top().value()) as u32;
        let img_width = img.width().value() as u32;
        let img_height = img.height().value() as u32;

        // Create empty image
        let mut result = if image.color().has_alpha() {
            DynamicImage::new_rgba8(width, height)
        } else {
            DynamicImage::new_rgb8(width, height)
        };
        ImageUtil::fill_image(&mut result, color.channels());

        // ***************************************
        // ************* DRAWING *****************
        // ***************************************

        // Borders
        self.draw_borders(
            &mut result,
            x_img,
            y_img,
            img_width,
            img_height,
            dpi,
            rotate,
        );

        let color = self
            .color
            .as_ref()
            .unwrap_or(&Color::new(0, 0, 0, 255))
            .clone();
        let rgba = Rgba(*color.channels());

        // Cut marks
        if let Some(m) = &self.cut_marks {
            let marks = m.to_px(dpi);
            let lw = marks.width().as_ref().map_or(1, |l| l.value() as i32);
            let lw2 = lw / 2;
            let offset = marks.height().as_ref().map_or(0, |l| l.value() as i32);
            let xmin = x_img as i32 - padding.left().value() as i32;
            let xmax = x_img as i32 + img_width as i32 + padding.right().value() as i32;
            let ymin = y_img as i32 - padding.top().value() as i32;
            let ymax = y_img as i32 + img_height as i32 + padding.bottom().value() as i32;

            // Top left
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(0, ymin - lw2).of_size((xmin - offset) as u32, lw as u32),
                rgba,
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmin - lw2, 0).of_size(lw as u32, (ymin - offset) as u32),
                rgba,
            );

            // Top right
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmax + offset, ymin - lw2)
                    .of_size((width as i32 - xmax - offset) as u32, lw as u32),
                rgba,
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmax - lw2, 0).of_size(lw as u32, (ymin - offset) as u32),
                rgba,
            );

            // Bottom left
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(0, ymax - lw2).of_size((xmin - offset) as u32, lw as u32),
                rgba,
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmin - lw2, ymax + offset)
                    .of_size(lw as u32, (height as i32 - ymax - offset) as u32),
                rgba,
            );

            // Bottom right
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmax + offset, ymax - lw2)
                    .of_size((width as i32 - xmax - offset) as u32, lw as u32),
                rgba,
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmax - lw2, ymax + offset)
                    .of_size(lw as u32, (height as i32 - ymax - offset) as u32),
                rgba,
            );
        }

        // Cut frame
        if let Some(f) = &self.cut_frame {
            let frame = f.to_px(dpi);
            let lw = frame.width().as_ref().map_or(1, |l| l.value() as i32);
            let lw2 = lw / 2;
            let offset = frame.height().as_ref().map_or(0, |l| l.value() as i32);
            let xmin = x_img as i32 - padding.left().value() as i32;
            let xmax = x_img as i32 + img_width as i32 + padding.right().value() as i32;
            let ymin = y_img as i32 - padding.top().value() as i32;
            let ymax = y_img as i32 + img_height as i32 + padding.bottom().value() as i32;

            // Top
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmin - offset, ymin - lw2)
                    .of_size(((xmax - xmin) + 2 * offset) as u32, lw as u32),
                rgba,
            );

            // Bottom
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmin - offset, ymax - lw2)
                    .of_size(((xmax - xmin) + 2 * offset) as u32, lw as u32),
                rgba,
            );

            // Left
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmin - lw2, ymin - offset)
                    .of_size(lw as u32, ((ymax - ymin) + 2 * offset) as u32),
                rgba,
            );

            // Right
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmax - lw2, ymin - offset)
                    .of_size(lw as u32, ((ymax - ymin) + 2 * offset) as u32),
                rgba,
            );
        }

        let pad_distance = Length::mm(2.0).to_px(dpi).value() as u32;
        // EXIF data
        if let Some(format) = &self.exif {
            let exif = ImageUtil::get_exif_map(&file);
            let font_size = self
                .exif_size
                .clone()
                .unwrap_or_else(|| Length::px(12))
                .to_px(dpi)
                .value();
            if let Ok(exif) = exif {
                let str = self.exif_string(&format, &exif);
                imageproc::drawing::draw_text_mut(
                    &mut result,
                    rgba,
                    x_img, //x_img - padding.left().value() as u32 + 5,
                    y_img + img_height + padding.bottom().value() as u32 + pad_distance,
                    rusttype::Scale::uniform(font_size as f32),
                    &self.fonts.default,
                    &str,
                )
            }
        }

        // Control element
        if let Some(patt) = &self.test_pattern {
            let borders = patt.to_px(dpi);
            let mut element = self.create_control_element(&borders);
            //let x = x_img + img_width + padding.right().value() as u32 - 5 - element.width();
            let x = x_img + img_width - element.width();
            let y = y_img + img_height + padding.bottom().value() as u32 + pad_distance;
            if result.height() < y + element.height() {
                element = element.crop_imm(0, 0, element.width(), result.height() - y);
            }
            result.copy_from(&element, x, y)?;
        }

        // ***************************************
        // ********* SCALE & COPY ORIGINAL *******
        // ***************************************
        let scaled = ImageUtil::scale_image(
            image,
            img.width().value() as u32,
            img.height().value() as u32,
            &ScaleMode::Stretch,
            filter,
            &color,
            self.incremental,
        )?;

        result.copy_from(&scaled, x_img, y_img)?;

        Ok(result)
    }
}

impl PrepareImage {
    fn check(&self) -> Result<(), Box<dyn Error>> {
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

    fn exif_string(&self, format: &str, exif: &HashMap<String, String>) -> String {
        let mut str = format.to_string();
        for (k, v) in exif.iter() {
            let key = format!("{{{}}}", k);
            str = str.replace(&key, v);
        }
        str
    }

    fn create_control_element(&self, sizes: &Borders) -> DynamicImage {
        let off_x = sizes.right().value() as i32;
        let off_y = sizes.left().value() as i32;
        let sx = sizes.top().value() as u32;
        let sy = sizes.bottom().value() as u32;
        let mut image =
            DynamicImage::new_rgb8(9 * sy + 10 * off_x as u32, 3 * sy + 4 * off_y as u32);
        ImageUtil::fill_image(&mut image, &[255, 255, 255, 255]);

        // CMY(K)
        for i in 0..5 {
            imageproc::drawing::draw_filled_rect_mut(
                &mut image,
                Rect::at(off_x + i * (sx as i32 + off_x), off_y).of_size(sx, sy),
                Rgba([i as u8 * 51, 255, 255, 255]),
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut image,
                Rect::at(
                    off_x + i * (sx as i32 + off_x),
                    off_y + 1 * (sy as i32 + off_y),
                )
                .of_size(sx, sy),
                Rgba([255, i as u8 * 51, 255, 255]),
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut image,
                Rect::at(
                    off_x + i * (sx as i32 + off_x),
                    off_y + 2 * (sy as i32 + off_y),
                )
                .of_size(sx, sy),
                Rgba([255, 255, i as u8 * 51, 255]),
            );
        }
        // RGB
        imageproc::drawing::draw_filled_rect_mut(
            &mut image,
            Rect::at(off_x + 5 * (sx as i32 + off_x), off_y).of_size(sx, sy),
            Rgba([255, 0, 0, 255]),
        );
        imageproc::drawing::draw_filled_rect_mut(
            &mut image,
            Rect::at(
                off_x + 5 * (sx as i32 + off_x),
                off_y + 1 * (sy as i32 + off_y),
            )
            .of_size(sx, sy),
            Rgba([0, 255, 0, 255]),
        );
        imageproc::drawing::draw_filled_rect_mut(
            &mut image,
            Rect::at(
                off_x + 5 * (sx as i32 + off_x),
                off_y + 2 * (sy as i32 + off_y),
            )
            .of_size(sx, sy),
            Rgba([0, 0, 255, 255]),
        );
        // Greyscale light
        imageproc::drawing::draw_filled_rect_mut(
            &mut image,
            Rect::at(off_x + 6 * (sx as i32 + off_x), off_y).of_size(sx, sy),
            Rgba([255, 255, 255, 255]),
        );
        imageproc::drawing::draw_filled_rect_mut(
            &mut image,
            Rect::at(
                off_x + 6 * (sx as i32 + off_x),
                off_y + 1 * (sy as i32 + off_y),
            )
            .of_size(sx, sy),
            Rgba([204, 204, 204, 255]),
        );
        imageproc::drawing::draw_filled_rect_mut(
            &mut image,
            Rect::at(
                off_x + 6 * (sx as i32 + off_x),
                off_y + 2 * (sy as i32 + off_y),
            )
            .of_size(sx, sy),
            Rgba([153, 153, 153, 255]),
        );
        // Greyscale dark
        imageproc::drawing::draw_filled_rect_mut(
            &mut image,
            Rect::at(off_x + 7 * (sx as i32 + off_x), off_y).of_size(sx, sy),
            Rgba([102, 102, 102, 255]),
        );
        imageproc::drawing::draw_filled_rect_mut(
            &mut image,
            Rect::at(
                off_x + 7 * (sx as i32 + off_x),
                off_y + 1 * (sy as i32 + off_y),
            )
            .of_size(sx, sy),
            Rgba([51, 51, 51, 255]),
        );
        imageproc::drawing::draw_filled_rect_mut(
            &mut image,
            Rect::at(
                off_x + 7 * (sx as i32 + off_x),
                off_y + 2 * (sy as i32 + off_y),
            )
            .of_size(sx, sy),
            Rgba([0, 0, 0, 255]),
        );
        // Vertical pattern
        for i in 0..(sx / 2) {
            let x = 2 * i as i32;
            imageproc::drawing::draw_filled_rect_mut(
                &mut image,
                Rect::at(off_x + 8 * (sx as i32 + off_x) + x, off_y).of_size(1, sy),
                Rgba([0, 0, 0, 255]),
            );
        }
        // Horizontal pattern
        for i in 0..(sy / 2) {
            let y = 2 * i as i32;
            imageproc::drawing::draw_filled_rect_mut(
                &mut image,
                Rect::at(
                    off_x + 8 * (sx as i32 + off_x),
                    off_y + 2 * (sy as i32 + off_y) + y,
                )
                .of_size(sx, 1),
                Rgba([0, 0, 0, 255]),
            );
        }
        // Crosshair
        {
            let x = off_x + 8 * (sx as i32 + off_x) + sx as i32 / 2;
            let y = off_y + 1 * (sy as i32 + off_y) + sy as i32 / 2;
            imageproc::drawing::draw_hollow_circle_mut(
                &mut image,
                (x, y),
                std::cmp::min(sx, sy) as i32 / 3,
                Rgba([0, 0, 0, 255]),
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut image,
                Rect::at(x - sx as i32 / 2, y).of_size(sx, 1),
                Rgba([0, 0, 0, 255]),
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut image,
                Rect::at(x, y - sy as i32 / 2).of_size(1, sy),
                Rgba([0, 0, 0, 255]),
            );
        }

        image
    }

    /// Returns calculated (image, framed, padding, margins).
    fn calc_sizes(
        &self,
        width: u32,
        height: u32,
        img_width: u32,
        img_height: u32,
        rotate: bool,
        dpi: f64,
    ) -> (FixSize, FixSize, Borders, Borders) {
        // Calculate maximum size of image + padding
        let framed = if let Some(framed) = &self.framed_size {
            Self::rotate_size(framed.to_px(dpi), rotate)
        } else {
            if let Some(margins) = &self.margins {
                let mar = Self::rotate_borders(margins.to_px(dpi), rotate);
                FixSize::px(
                    width as i32 - mar.right().value() as i32 - mar.left().value() as i32,
                    height as i32 - mar.top().value() as i32 - mar.bottom().value() as i32,
                )
            } else {
                let img = Self::rotate_size(self.image_size.as_ref().unwrap().to_px(dpi), rotate);
                let pad = Self::rotate_borders(self.padding.as_ref().unwrap().to_px(dpi), rotate);
                FixSize::px(
                    img.width().value() as i32
                        + pad.right().value() as i32
                        + pad.left().value() as i32,
                    img.height().value() as i32
                        + pad.top().value() as i32
                        + pad.bottom().value() as i32,
                )
            }
        };

        // Calculate maximum size of image (without padding)
        let image = if let Some(image) = &self.framed_size {
            Self::rotate_size(image.to_px(dpi), rotate)
        } else {
            let pad = Self::rotate_borders(self.padding.as_ref().unwrap().to_px(dpi), rotate);
            FixSize::px(
                framed.width().value() as i32
                    - pad.right().value() as i32
                    - pad.left().value() as i32,
                framed.height().value() as i32
                    - pad.top().value() as i32
                    - pad.bottom().value() as i32,
            )
        };
        // Calculate padding
        let padding = if let Some(pad) = &self.padding {
            Self::rotate_borders(pad.to_px(dpi), rotate)
        } else {
            let hor = (framed.width().value() as i32 - image.width().value() as i32) / 2;
            let ver = (framed.height().value() as i32 - image.height().value() as i32) / 2;
            Borders::px(ver, hor, ver, hor)
        };

        // Resize for original aspect ratio
        let (scaled_width, scaled_height) = {
            let orig_aspect = img_width as f64 / img_height as f64;
            let out_aspect = image.width().value() / image.height().value();
            if orig_aspect >= out_aspect {
                // wider
                (
                    image.width().value().round() as i32,
                    (image.height().value() * out_aspect / orig_aspect).round() as i32,
                )
            } else {
                // higher
                (
                    (image.width().value() * orig_aspect / out_aspect).round() as i32,
                    image.height().value().round() as i32,
                )
            }
        };

        // Calculate actual size of image
        let image = FixSize::px(scaled_width, scaled_height);

        // Calculate actual size of image + padding
        let framed = FixSize::px(
            scaled_width + padding.left().value() as i32 + padding.right().value() as i32,
            scaled_height + padding.top().value() as i32 + padding.bottom().value() as i32,
        );

        // Calculate actual margine
        let margins = if let Some(mar_orig) = &self.margins {
            let mar = Self::rotate_borders(mar_orig.to_px(dpi), rotate);
            let diff_hor = (mar.right().value() as i32 - mar.left().value() as i32) / 2;
            let diff_ver = (mar.top().value() as i32 - mar.bottom().value() as i32) / 2;
            let hor = (width as i32 - framed.width().value() as i32) / 2;
            let ver = (height as i32 - framed.height().value() as i32) / 2;
            Borders::px(
                ver + diff_ver,
                hor + diff_hor,
                ver - diff_ver,
                hor - diff_hor,
            )
        } else {
            let hor = (width as i32 - framed.width().value() as i32) / 2;
            let ver = (height as i32 - framed.height().value() as i32) / 2;
            Borders::px(ver, hor, ver, hor)
        };

        (image, framed, padding, margins)
    }

    fn rotate_size(size: FixSize, rotate: bool) -> FixSize {
        if rotate {
            size.rotate_90()
        } else {
            size
        }
    }
    fn rotate_borders(borders: Borders, _rotate: bool) -> Borders {
        borders
    }

    fn draw_borders(
        &self,
        image: &mut DynamicImage,
        image_x: u32,
        image_y: u32,
        image_width: u32,
        image_height: u32,
        dpi: f64,
        rotate: bool,
    ) {
        if let Some(b) = &self.border {
            let bor = Self::rotate_borders(b.to_px(dpi), rotate);
            let color = Rgba(
                self.border_color
                    .as_ref()
                    .map_or([0_u8, 0, 0, 255], |c| *c.channels()),
            );
            imageproc::drawing::draw_filled_rect_mut(
                image,
                Rect::at(
                    image_x as i32 - bor.left().value() as i32,
                    image_y as i32 - bor.top().value() as i32,
                )
                .of_size(
                    image_width + bor.left().value() as u32 + bor.right().value() as u32,
                    image_height + bor.top().value() as u32 + bor.bottom().value() as u32,
                ),
                color,
            );
        }
    }
}
