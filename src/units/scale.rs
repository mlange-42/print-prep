//! Relative scale.

use crate::{ParseEnumError, ParseStructError};
use std::error::Error;
use std::str::FromStr;

/// Scaling modes
#[derive(Debug, PartialEq)]
pub enum ScaleMode {
    /// Keeps the original aspect ratio.
    /// The resulting image may be smaller than given by scale/size in one dimension.
    Keep,
    /// Keeps the original aspect ratio.
    /// The resulting image has exactly the given size, but additional space is filled uniformly.
    Fill,
    /// Keeps the original aspect ratio.
    /// The resulting image has exactly the given size, but surplus image space is cropped.
    Crop,
    /// Aspect ratio is changed.
    /// The resulting image has exactly the given size, and the image is stretched.
    Stretch,
}

impl FromStr for ScaleMode {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fill" => Ok(ScaleMode::Fill),
            "crop" => Ok(ScaleMode::Crop),
            "keep" => Ok(ScaleMode::Keep),
            "stretch" => Ok(ScaleMode::Stretch),
            _ => Err(ParseEnumError(format!(
                "`{}` is not a valid scale mode. Must be one of `(keep|fill|crop|stretch)`",
                s
            ))),
        }
    }
}

/// Relative scaling parameters.
///
/// Can be parsed from stings of format `width/height` or `scale`.
/// `width` and `height´ can be fractions or percentages.
/// Examples:
/// ```ignore
/// 50%
/// 20%/50%
/// 0.6/0.8
/// ```
///
/// `.` can be used as placeholder.
/// Examples:
/// ```ignore
/// ./20%
/// ```
#[derive(Debug, PartialEq)]
pub struct Scale {
    width: f32,
    height: f32,
}
impl Scale {
    /// Width of this scaling.
    pub fn width(&self) -> f32 {
        self.width
    }
    /// Height of this scaling.
    pub fn height(&self) -> f32 {
        self.height
    }
}

impl FromStr for Scale {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("/").collect();
        if parts.len() < 1 || parts.len() > 2 {
            return Err(Box::new(ParseStructError(format!(
                "Unexpected scale format in {}, expects `width/height` or `scale`",
                s
            ))));
        }
        let mut width = if parts[0] == "." {
            None
        } else {
            if parts[0].ends_with('%') {
                Some(parts[0][..(parts[0].len() - 1)].parse::<f32>()? * 0.01)
            } else {
                Some(parts[0].parse()?)
            }
        };
        let mut height = if parts.len() < 2 || parts[1] == "." {
            None
        } else {
            if parts[1].ends_with('%') {
                Some(parts[1][..(parts[1].len() - 1)].parse::<f32>()? * 0.01)
            } else {
                Some(parts[1].parse()?)
            }
        };
        if width.is_none() && height.is_none() {
            return Err(Box::new(ParseStructError(format!(
                "Unable to parse scale from {}, at least one of width or height must be given",
                s
            ))));
        }
        if width.is_none() {
            width = Some(height.unwrap())
        }
        if height.is_none() {
            height = Some(width.unwrap())
        }
        Ok(Scale {
            width: width.unwrap(),
            height: height.unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::units::scale::Scale;

    #[test]
    fn parse_scale() {
        let str = "50%/100%";
        let scale: Scale = str.parse().unwrap();

        assert_eq!(scale.width, 0.5);
        assert_eq!(scale.height, 1.0);
    }
}
