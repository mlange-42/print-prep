//! Absolute scale.

use crate::units::length::{Length, LengthUnit};
use crate::ParseStructError;
use std::error::Error;
use std::str::FromStr;

/// Absolute scaling parameters.
///
/// Can be parsed from stings of format `width/height`.
/// `width` and `height´ can be in units px, cm or inch.
/// Examples:
/// <pre>
/// 10cm/5cm
/// 1024px/512px
/// 5in/3in
/// </pre>
///
/// `.` can be used as placeholder.
/// Examples:
/// <pre>
/// 15cm/.
/// ./512px
/// </pre>
#[derive(Debug, PartialEq)]
pub struct Size {
    width: Option<Length>,
    height: Option<Length>,
}

impl Size {
    pub fn new(width: Option<Length>, height: Option<Length>) -> Result<Self, ParseStructError> {
        if width.is_none() && height.is_none() {
            return Err(ParseStructError(
                "Unable to create size, at least one of width or height must be given".to_string(),
            ));
        }
        Ok(Size { width, height })
    }
    /// Width of this size.
    pub fn width(&self) -> &Option<Length> {
        &self.width
    }
    /// Height of this size.
    pub fn height(&self) -> &Option<Length> {
        &self.height
    }
    /// Converts this size to another unit.
    pub fn to(&self, unit: &LengthUnit, dpi: f32) -> Size {
        Size {
            width: self.width.as_ref().and_then(|w| Some(w.to(unit, dpi))),
            height: self.height.as_ref().and_then(|w| Some(w.to(unit, dpi))),
        }
    }
    /// Does this size require a dpi value for conversion to px?
    pub fn needs_dpi(&self) -> bool {
        let mut needs = false;
        if let Some(w) = &self.width {
            if w.needs_dpi() {
                needs = true;
            }
        };
        if let Some(h) = &self.height {
            if h.needs_dpi() {
                needs = true;
            }
        };
        needs
    }
}

impl FromStr for Size {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("/").collect();
        if parts.len() != 2 {
            return Err(Box::new(ParseStructError(format!(
                "Unexpected size format in {}, expects `width/height`",
                s
            ))));
        }
        let width = if parts[0] == "." {
            None
        } else {
            Some(parts[0].parse()?)
        };
        let height = if parts[1] == "." {
            None
        } else {
            Some(parts[1].parse()?)
        };
        if width.is_none() && height.is_none() {
            return Err(Box::new(ParseStructError(format!(
                "Unable to parse size from {}, at least one of width or height must be given",
                s
            ))));
        }
        Ok(Size { width, height })
    }
}

#[cfg(test)]
mod test {
    use crate::units::length::LengthUnit;
    use crate::units::size::Size;

    #[test]
    fn parse_size() {
        let str = "10cm/5cm";
        let size: Size = str.parse().unwrap();

        assert_eq!(size.width.as_ref().unwrap().value(), 10.0);
        assert_eq!(size.height.as_ref().unwrap().value(), 5.0);
        assert_eq!(size.width.as_ref().unwrap().unit(), &LengthUnit::Cm);
        assert_eq!(size.height.as_ref().unwrap().unit(), &LengthUnit::Cm);
    }

    #[test]
    fn parse_size_opt() {
        let str = "10in/.";
        let size: Size = str.parse().unwrap();

        assert_eq!(size.width.as_ref().unwrap().value(), 10.0);
        assert_eq!(size.width.as_ref().unwrap().unit(), &LengthUnit::Inch);
        assert!(size.height.is_none());
    }
}
