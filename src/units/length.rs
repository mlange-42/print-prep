//! Length units and conversions

use crate::ParseEnumError;
use std::error::Error;
use std::str::FromStr;

/// A length with unit.
#[derive(Debug, PartialEq)]
pub struct Length {
    value: f32,
    unit: LengthUnit,
}

impl Length {
    /// The length value.
    pub fn value(&self) -> f32 {
        self.value
    }
    /// The length unit.
    pub fn unit(&self) -> &LengthUnit {
        &self.unit
    }
    /// Creates a new length in centimeters.
    pub fn cm(value: f32) -> Self {
        Length {
            value,
            unit: LengthUnit::Cm,
        }
    }
    /// Creates a new length in inches.
    pub fn inch(value: f32) -> Self {
        Length {
            value,
            unit: LengthUnit::Inch,
        }
    }
    /// Creates a new length in pixels.
    pub fn px(value: i32) -> Self {
        Length {
            value: value as f32,
            unit: LengthUnit::Px,
        }
    }
    /// Converts this length to another unit.
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
    /// Does this length require a dpi value for conversion to px?
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

/// Trait to convert numbers to lengths with unit.
pub trait ToLength {
    /// Converts the number to a length in centimeters.
    fn cm(&self) -> Length;
    /// Converts the number to a length in inches.
    fn inch(&self) -> Length;
    /// Converts the number to a length in pixels.
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

/// Length units.
#[derive(Debug, PartialEq, Clone)]
pub enum LengthUnit {
    /// Pixels.
    Px,
    /// Centimeters.
    Cm,
    /// Inches.
    Inch,
}
impl LengthUnit {
    /// Does this unit require a dpi value vor conversion to px?
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
    use crate::units::length::{Length, LengthUnit, ToLength};
    use crate::units::size::Size;

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

        assert_eq!(size.width().as_ref().unwrap().value, 10.0);
        assert_eq!(size.height().as_ref().unwrap().value, 5.0);
        assert_eq!(size.width().as_ref().unwrap().unit, LengthUnit::Cm);
        assert_eq!(size.height().as_ref().unwrap().unit, LengthUnit::Cm);
    }

    #[test]
    fn parse_size_opt() {
        let str = "10in/.";
        let size: Size = str.parse().unwrap();

        assert_eq!(size.width().as_ref().unwrap().value, 10.0);
        assert_eq!(size.width().as_ref().unwrap().unit, LengthUnit::Inch);
        assert!(size.height().is_none());
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
