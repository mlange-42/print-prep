//! Absolute scale.

use crate::units::length::{Length, LengthUnit};
use crate::ParseStructError;
use std::error::Error;
use std::fmt;
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
#[derive(Debug, PartialEq, Clone)]
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
    pub fn to(&self, unit: &LengthUnit, dpi: f64) -> Size {
        Size {
            width: self.width.as_ref().and_then(|w| Some(w.to(unit, dpi))),
            height: self.height.as_ref().and_then(|w| Some(w.to(unit, dpi))),
        }
    }
    /// Rotates this size by 90° clockwise (i.e. swaps width and height).
    pub fn rotate_90(&self) -> Size {
        Size::new(self.height.clone(), self.width.clone()).unwrap()
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
impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str1 = self
            .width
            .as_ref()
            .map_or(".".to_string(), |l| l.to_string());
        let str2 = self
            .height
            .as_ref()
            .map_or(".".to_string(), |l| l.to_string());
        write!(f, "{}/{}", str1, str2)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FixSize {
    width: Length,
    height: Length,
}

impl FixSize {
    pub fn new(width: Length, height: Length) -> Self {
        FixSize { width, height }
    }
    pub fn px(width: i32, height: i32) -> Self {
        FixSize {
            width: Length::px(width),
            height: Length::px(height),
        }
    }
    /// Width of this size.
    pub fn width(&self) -> &Length {
        &self.width
    }
    /// Height of this size.
    pub fn height(&self) -> &Length {
        &self.height
    }
    /// Converts this size to another unit.
    pub fn to(&self, unit: &LengthUnit, dpi: f64) -> FixSize {
        FixSize {
            width: self.width.to(unit, dpi),
            height: self.height.to(unit, dpi),
        }
    }
    /// Rotates this size by 90° clockwise (i.e. swaps width and height).
    pub fn rotate_90(&self) -> FixSize {
        FixSize::new(self.height.clone(), self.width.clone())
    }
    /// Does this size require a dpi value for conversion to px?
    pub fn needs_dpi(&self) -> bool {
        self.width.needs_dpi() || self.height.needs_dpi()
    }
}

impl FromStr for FixSize {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("/").collect();
        if parts.len() != 2 {
            return Err(Box::new(ParseStructError(format!(
                "Unexpected size format in {}, expects `width/height`",
                s
            ))));
        }
        let width = parts[0].parse()?;
        let height = parts[1].parse()?;
        Ok(FixSize { width, height })
    }
}
impl fmt::Display for FixSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.width, self.height)
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
    #[test]
    fn rotate() {
        let str = "10in/.";
        let size: Size = str.parse().unwrap();
        let rot = size.rotate_90();
        assert_eq!(rot.to_string(), "./10in");
    }
    #[test]
    fn display() {
        let str = "10in/.";
        let size: Size = str.parse().unwrap();
        assert_eq!(size.to_string(), str);

        let str = "15cm/10cm";
        let size: Size = str.parse().unwrap();
        assert_eq!(size.to_string(), str);
    }
}
