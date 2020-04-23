//! `print-prep` operations
use image::DynamicImage;
use std::error::Error;

pub mod scale;

use crate::{ParseEnumError, ParseStructError};
pub use scale::ScaleImage;
use std::str::FromStr;

pub trait ImageOperation {
    fn execute(&mut self, image: &DynamicImage) -> Result<DynamicImage, Box<dyn Error>>;
}

#[derive(Debug, PartialEq)]
pub struct Size {
    width: Length,
    height: Length,
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

        Ok(Size {
            width: parts[0].parse()?,
            height: parts[1].parse()?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Length {
    value: f32,
    unit: LengthUnit,
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

#[derive(Debug, PartialEq)]
pub enum LengthUnit {
    Px,
    Cm,
    Inch,
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
    use crate::op::{Length, LengthUnit};

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
}
