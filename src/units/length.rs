//! Length units and conversions

use crate::ParseEnumError;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// A length with unit.
#[derive(PartialEq, Clone)]
pub struct Length {
    value: f64,
    unit: LengthUnit,
}

impl Length {
    /// The length value.
    pub fn value(&self) -> f64 {
        self.value
    }
    /// The length unit.
    pub fn unit(&self) -> &LengthUnit {
        &self.unit
    }

    /// Creates a new length in centimeters.
    pub fn new(value: f64, unit: LengthUnit) -> Self {
        let v = if unit == LengthUnit::Px {
            value.round()
        } else {
            value
        };
        Length { value: v, unit }
    }
    /// Creates a new length in centimeters.
    pub fn cm(value: f64) -> Self {
        Length {
            value,
            unit: LengthUnit::Cm,
        }
    }
    /// Creates a new length in millimeters.
    pub fn mm(value: f64) -> Self {
        Length {
            value,
            unit: LengthUnit::Mm,
        }
    }
    /// Creates a new length in inches.
    pub fn inch(value: f64) -> Self {
        Length {
            value,
            unit: LengthUnit::Inch,
        }
    }
    /// Creates a new length in pixels.
    pub fn px(value: i32) -> Self {
        Length {
            value: value as f64,
            unit: LengthUnit::Px,
        }
    }
    /// Converts this length to pixels.
    pub fn to_px(&self, dpi: f64) -> Length {
        self.to(&LengthUnit::Px, dpi)
    }
    /// Converts this length to another unit.
    pub fn to(&self, unit: &LengthUnit, dpi: f64) -> Length {
        if &self.unit == unit {
            self.clone()
        } else {
            Length::new(
                self.value * self.unit.metric_factor(dpi) / unit.metric_factor(dpi),
                unit.clone(),
            )
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

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

impl fmt::Debug for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

/// Trait to convert numbers to lengths with unit.
pub trait ToLength {
    /// Converts the number to a length in centimeters.
    fn cm(&self) -> Length;
    /// Converts the number to a length in millimeters.
    fn mm(&self) -> Length;
    /// Converts the number to a length in inches.
    fn inch(&self) -> Length;
    /// Converts the number to a length in pixels.
    fn px(&self) -> Length;
}
impl ToLength for f64 {
    fn cm(&self) -> Length {
        Length::cm(*self)
    }
    fn mm(&self) -> Length {
        Length::mm(*self)
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
        Length::cm(*self as f64)
    }
    fn mm(&self) -> Length {
        Length::mm(*self as f64)
    }
    fn inch(&self) -> Length {
        Length::inch(*self as f64)
    }
    fn px(&self) -> Length {
        Length::px(*self)
    }
}

const INCH_TO_METERS: f64 = 0.0254;

/// Length units.
#[derive(Debug, PartialEq, Clone)]
pub enum LengthUnit {
    /// Pixels.
    Px,
    /// Centimeters.
    Cm,
    /// Millimeters.
    Mm,
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

    /// Converts this length to another unit.
    pub fn metric_factor(&self, dpi: f64) -> f64 {
        match self {
            LengthUnit::Cm => 0.01,
            LengthUnit::Mm => 0.001,
            LengthUnit::Inch => 0.0254,
            LengthUnit::Px => INCH_TO_METERS / dpi,
        }
    }
}
impl FromStr for LengthUnit {
    type Err = ParseEnumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "px" => Ok(LengthUnit::Px),
            "cm" => Ok(LengthUnit::Cm),
            "mm" => Ok(LengthUnit::Mm),
            "in" => Ok(LengthUnit::Inch),
            _ => Err(ParseEnumError(format!(
                "`{}` is not a valid length unit. Must be one of `(px|cm|mm|in)`",
                s
            ))),
        }
    }
}
impl fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LengthUnit::Cm => "cm",
                LengthUnit::Mm => "mm",
                LengthUnit::Inch => "in",
                LengthUnit::Px => "px",
            }
        )
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
        let mm = 2540.mm();
        let cm = 254.cm();
        let inch = 100.inch();
        let px = 30000.px();

        assert_eq!(mm.to(&LengthUnit::Mm, 300.0), mm);
        assert_eq!(cm.to(&LengthUnit::Cm, 300.0), cm);
        assert_eq!(inch.to(&LengthUnit::Inch, 300.0), inch);
        assert_eq!(px.to(&LengthUnit::Px, 300.0), px);

        assert_eq!(cm.to(&LengthUnit::Mm, 300.0), mm);
        assert_eq!(cm.to(&LengthUnit::Inch, 300.0), inch);
        assert_eq!(cm.to(&LengthUnit::Px, 300.0), px);

        assert_eq!(inch.to(&LengthUnit::Cm, 300.0), cm);
        assert_eq!(inch.to(&LengthUnit::Mm, 300.0), mm);
        assert_eq!(inch.to(&LengthUnit::Px, 300.0), px);

        assert!((px.to(&LengthUnit::Cm, 300.0).value - cm.value).abs() < 0.000001);
        assert!((px.to(&LengthUnit::Mm, 300.0).value - mm.value).abs() < 0.000001);
        assert!((px.to(&LengthUnit::Inch, 300.0).value - inch.value).abs() < 0.000001);
    }

    #[test]
    fn display() {
        let cm = 254.cm();
        let inch = 100.inch();
        let px = 30000.px();

        assert_eq!(cm.to_string(), "254cm");
        assert_eq!(inch.to_string(), "100in");
        assert_eq!(px.to_string(), "30000px");
    }
}
