//! Prepare images for printing.

use crate::cli::parse;
use crate::op::{ImageIoOperation, ImageOperation};
use crate::units::color::Color;
use crate::units::{format, LengthUnit, ScaleMode, Size};
use crate::units::{Borders, FixSize};
use crate::util::ImageUtil;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use imageproc::rect::Rect;
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

    /// Cut marks with offset. Format <line-width>/<offset>. Use alternative to `--cut-frame`.
    #[structopt(name = "cut-marks", long)]
    pub cut_marks: Option<Size>,

    /// Cut frame. Format <line-width>/<extend>. Use alternative to `--cut-marks`.
    #[structopt(name = "cut-frame", long)]
    pub cut_frame: Option<Size>,

    /// Cut marks or frame color. Default: black.
    #[structopt(name = "cut-color", long)]
    pub cut_color: Option<Color>,

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
    pub format: FixSize,

    /// Maximum image size, excl. padding.
    #[structopt(name = "image-size", long)]
    pub image_size: Option<FixSize>,

    /// Maximum image size, incl. padding.
    #[structopt(name = "framed-size", long)]
    pub framed_size: Option<FixSize>,

    /// Padding.
    #[structopt(long)]
    pub padding: Option<Borders>,

    /// Minimum margins.
    #[structopt(long)]
    pub margins: Option<Borders>,

    /// Border width. Default none.
    #[structopt(long)]
    pub border: Option<Borders>,

    /// Border color. Default black.
    #[structopt(name = "border-color", long)]
    pub border_color: Option<Color>,

    /// Enable incremental scaling.
    /// For scaling to small sizes, scales down in multiple steps, to 50% per step, averaging over 2x2 pixels.
    #[structopt(long)]
    pub incremental: bool,

    /// Background color. Default `white`.
    #[structopt(short, long)]
    pub bg: Option<Color>,

    /// Prevents rotation of portrait format images (or of landscape format images if output is portrait).
    #[structopt(name = "no-rotation", long)]
    pub no_rotation: bool,
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

    fn rotate_size(size: FixSize, _rotate: bool) -> FixSize {
        // TODO: Can probably be removed
        /*if rotate {
            size.rotate_90()
        } else {
            size
        }*/
        size
    }

    fn rotate_borders(borders: Borders, _rotate: bool) -> Borders {
        // TODO: Can probably be removed
        /*if rotate {
            borders.rotate_90()
        } else {
            borders
        }*/
        borders
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
        let (img, frame, padding, margins) =
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
        if let Some(b) = &self.border {
            let bor = Self::rotate_borders(b.to_px(dpi), rotate);
            let color = Rgba(
                self.border_color
                    .as_ref()
                    .map_or([0_u8, 0, 0, 255], |c| *c.channels()),
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(
                    x_img as i32 - bor.left().value() as i32,
                    y_img as i32 - bor.top().value() as i32,
                )
                .of_size(
                    img_width + bor.left().value() as u32 + bor.right().value() as u32,
                    img_height + bor.top().value() as u32 + bor.bottom().value() as u32,
                ),
                color,
            );
        }

        // Cut marks
        if let Some(m) = &self.cut_marks {
            let marks = m.to_px(dpi);
            let color = Rgba(
                self.cut_color
                    .as_ref()
                    .map_or([0_u8, 0, 0, 255], |c| *c.channels()),
            );
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
                color,
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmin - lw2, 0).of_size(lw as u32, (ymin - offset) as u32),
                color,
            );

            // Top right
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmax + offset, ymin - lw2)
                    .of_size((width as i32 - xmax - offset) as u32, lw as u32),
                color,
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmax - lw2, 0).of_size(lw as u32, (ymin - offset) as u32),
                color,
            );

            // Bottom left
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(0, ymax - lw2).of_size((xmin - offset) as u32, lw as u32),
                color,
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmin - lw2, ymax + offset)
                    .of_size(lw as u32, (height as i32 - ymax - offset) as u32),
                color,
            );

            // Bottom right
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmax + offset, ymax - lw2)
                    .of_size((width as i32 - xmax - offset) as u32, lw as u32),
                color,
            );
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmax - lw2, ymax + offset)
                    .of_size(lw as u32, (height as i32 - ymax - offset) as u32),
                color,
            );
        }

        // Cut frame
        if let Some(f) = &self.cut_frame {
            let frame = f.to_px(dpi);
            let color = Rgba(
                self.cut_color
                    .as_ref()
                    .map_or([0_u8, 0, 0, 255], |c| *c.channels()),
            );
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
                color,
            );

            // Bottom
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmin - offset, ymax - lw2)
                    .of_size(((xmax - xmin) + 2 * offset) as u32, lw as u32),
                color,
            );

            // Left
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmin - lw2, ymin - offset)
                    .of_size(lw as u32, ((ymax - ymin) + 2 * offset) as u32),
                color,
            );

            // Right
            imageproc::drawing::draw_filled_rect_mut(
                &mut result,
                Rect::at(xmax - lw2, ymin - offset)
                    .of_size(lw as u32, ((ymax - ymin) + 2 * offset) as u32),
                color,
            );
        }
        /*
        let pad_color = Rgba([0, 0, 0, 255]);
        imageproc::drawing::draw_hollow_rect_mut(
            &mut result,
            Rect::at(margins.left().value() as i32, margins.top().value() as i32)
                .of_size(frame.width().value() as u32, frame.height().value() as u32),
            pad_color,
        );*/

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
