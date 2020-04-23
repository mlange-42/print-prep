//! Length units and conversions

use crate::{ParseEnumError, ParseStructError};
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Scale {
    width: f32,
    height: f32,
}
impl Scale {
    pub fn width(&self) -> f32 {
        self.width
    }
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

#[derive(Debug, PartialEq)]
pub enum ScaleMode {
    Fill,
    Crop,
    Keep,
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
    pub fn width(&self) -> &Option<Length> {
        &self.width
    }
    pub fn height(&self) -> &Option<Length> {
        &self.height
    }
    pub fn to(&self, unit: &LengthUnit, dpi: f32) -> Size {
        Size {
            width: self.width.as_ref().and_then(|w| Some(w.to(unit, dpi))),
            height: self.height.as_ref().and_then(|w| Some(w.to(unit, dpi))),
        }
    }
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

#[derive(Debug, PartialEq)]
pub struct Length {
    value: f32,
    unit: LengthUnit,
}

impl Length {
    pub fn value(&self) -> f32 {
        self.value
    }
    pub fn unit(&self) -> &LengthUnit {
        &self.unit
    }
    pub fn cm(value: f32) -> Self {
        Length {
            value,
            unit: LengthUnit::Cm,
        }
    }
    pub fn inch(value: f32) -> Self {
        Length {
            value,
            unit: LengthUnit::Inch,
        }
    }
    pub fn px(value: i32) -> Self {
        Length {
            value: value as f32,
            unit: LengthUnit::Px,
        }
    }
    pub fn to(&self, unit: &LengthUnit, dpi: f32) -> Length {
        match self.unit {
            LengthUnit::Cm => match unit {
                LengthUnit::Cm => Length::cm(self.value),
                LengthUnit::Inch => Length::inch(self.value * CM_TO_INCH),
                LengthUnit::Px => Length::px((self.value * CM_TO_INCH * dpi).round() as i32),
            },
            LengthUnit::Inch => match unit {
                LengthUnit::Cm => Length::cm(self.value * INCH_TO_CM),
                LengthUnit::Inch => Length::inch(self.value),
                LengthUnit::Px => Length::px((self.value * dpi).round() as i32),
            },
            LengthUnit::Px => match unit {
                LengthUnit::Cm => Length::cm(self.value * INCH_TO_CM / dpi),
                LengthUnit::Inch => Length::inch(self.value / dpi),
                LengthUnit::Px => Length::px(self.value.round() as i32),
            },
        }
    }
    pub fn needs_dpi(&self) -> bool {
        self.unit.needs_dpi()
    }
}

impl FromStr for Length {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s.len() - 2;
        let unit_str = &s[pos..];
        let (unit, val_str) = if unit_str.chars().all(|c| char::is_alphabetic(c)) {
            (unit_str.parse()?, &s[..pos])
        } else {
            (LengthUnit::Px, s)
        };

        let value = val_str.parse()?;

        Ok(Length { value, unit })
    }
}

pub trait ToLength {
    fn cm(&self) -> Length;
    fn inch(&self) -> Length;
    fn px(&self) -> Length;
}
impl ToLength for f32 {
    fn cm(&self) -> Length {
        Length::cm(*self)
    }
    fn inch(&self) -> Length {
        Length::inch(*self)
    }
    fn px(&self) -> Length {
        Length::px(*self as i32)
    }
}
impl ToLength for i32 {
    fn cm(&self) -> Length {
        Length::cm(*self as f32)
    }
    fn inch(&self) -> Length {
        Length::inch(*self as f32)
    }
    fn px(&self) -> Length {
        Length::px(*self)
    }
}

const INCH_TO_CM: f32 = 2.54;
const CM_TO_INCH: f32 = 1.0 / 2.54;

#[derive(Debug, PartialEq)]
pub enum LengthUnit {
    Px,
    Cm,
    Inch,
}
impl LengthUnit {
    pub fn needs_dpi(&self) -> bool {
        match self {
            LengthUnit::Px => false,
            _ => true,
        }
    }
}
impl FromStr for LengthUnit {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "px" => Ok(LengthUnit::Px),
            "cm" => Ok(LengthUnit::Cm),
            "in" => Ok(LengthUnit::Inch),
            _ => Err(ParseEnumError(format!(
                "`{}` is not a valid length unit. Must be one of `(px|cm|in)`",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::units::length::{Length, LengthUnit, Scale, Size, ToLength};

    #[test]
    fn parse_length() {
        let str = "1024";
        let len: Length = str.parse().unwrap();
        assert_eq!(len.value, 1024.0);
        assert_eq!(len.unit, LengthUnit::Px);

        let str = "5cm";
        let len: Length = str.parse().unwrap();
        assert_eq!(len.value, 5.0);
        assert_eq!(len.unit, LengthUnit::Cm);

        let str = "10in";
        let len: Length = str.parse().unwrap();
        assert_eq!(len.value, 10.0);
        assert_eq!(len.unit, LengthUnit::Inch);
    }

    #[test]
    fn parse_size() {
        let str = "10cm/5cm";
        let size: Size = str.parse().unwrap();

        assert_eq!(size.width.as_ref().unwrap().value, 10.0);
        assert_eq!(size.height.as_ref().unwrap().value, 5.0);
        assert_eq!(size.width.as_ref().unwrap().unit, LengthUnit::Cm);
        assert_eq!(size.height.as_ref().unwrap().unit, LengthUnit::Cm);
    }

    #[test]
    fn parse_scale() {
        let str = "50%/100%";
        let scale: Scale = str.parse().unwrap();

        assert_eq!(scale.width, 0.5);
        assert_eq!(scale.height, 1.0);
    }

    #[test]
    fn parse_size_opt() {
        let str = "10in/.";
        let size: Size = str.parse().unwrap();

        assert_eq!(size.width.as_ref().unwrap().value, 10.0);
        assert_eq!(size.width.as_ref().unwrap().unit, LengthUnit::Inch);
        assert!(size.height.is_none());
    }

    #[test]
    fn parse_numbers() {
        let len = 10.cm();

        assert_eq!(len.value, 10.0);
        assert_eq!(len.unit, LengthUnit::Cm);
    }

    #[test]
    fn unit_conversion() {
        let cm = 254.cm();
        let inch = 100.inch();
        let px = 30000.px();

        assert_eq!(cm.to(&LengthUnit::Cm, 300.0), cm);
        assert_eq!(inch.to(&LengthUnit::Inch, 300.0), inch);
        assert_eq!(px.to(&LengthUnit::Px, 300.0), px);

        assert_eq!(cm.to(&LengthUnit::Inch, 300.0), inch);
        assert_eq!(cm.to(&LengthUnit::Px, 300.0), px);

        assert_eq!(inch.to(&LengthUnit::Cm, 300.0), cm);
        assert_eq!(inch.to(&LengthUnit::Px, 300.0), px);

        assert_eq!(px.to(&LengthUnit::Cm, 300.0), cm);
        assert_eq!(px.to(&LengthUnit::Inch, 300.0), inch);
    }
}
