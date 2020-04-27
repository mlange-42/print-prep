//! Borders/rectangles.

use crate::units::{Length, LengthUnit};
use crate::ParseStructError;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Borders {
    top: Length,
    right: Length,
    bottom: Length,
    left: Length,
}

impl Borders {
    pub fn each(top: Length, right: Length, bottom: Length, left: Length) -> Self {
        Borders {
            top,
            right,
            bottom,
            left,
        }
    }
    pub fn px(top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Borders {
            top: Length::px(top),
            right: Length::px(right),
            bottom: Length::px(bottom),
            left: Length::px(left),
        }
    }
    pub fn all(border: Length) -> Self {
        Borders {
            top: border.clone(),
            right: border.clone(),
            bottom: border.clone(),
            left: border.clone(),
        }
    }
    pub fn top(&self) -> &Length {
        &self.top
    }
    pub fn right(&self) -> &Length {
        &self.right
    }
    pub fn bottom(&self) -> &Length {
        &self.bottom
    }
    pub fn left(&self) -> &Length {
        &self.left
    }

    /// Converts these borders to pixels.
    pub fn to_px(&self, dpi: f64) -> Borders {
        self.to(&LengthUnit::Px, dpi)
    }
    /// Converts these borders to another unit.
    pub fn to(&self, unit: &LengthUnit, dpi: f64) -> Borders {
        Borders {
            top: self.top.to(unit, dpi),
            right: self.right.to(unit, dpi),
            bottom: self.bottom.to(unit, dpi),
            left: self.left.to(unit, dpi),
        }
    }
    /// Rotates these borders by 90° clockwise.
    pub fn rotate_90(&self) -> Borders {
        Borders::each(
            self.left.clone(),
            self.top.clone(),
            self.right.clone(),
            self.bottom.clone(),
        )
    }
    /// Rotates these borders by 27° clockwise.
    pub fn rotate_270(&self) -> Borders {
        Borders::each(
            self.right.clone(),
            self.bottom.clone(),
            self.left.clone(),
            self.top.clone(),
        )
    }
    /// Do these borders require a dpi value for conversion to px?
    pub fn needs_dpi(&self) -> bool {
        self.top.needs_dpi()
            || self.right.needs_dpi()
            || self.bottom.needs_dpi()
            || self.left.needs_dpi()
    }
}

impl FromStr for Borders {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split("/").collect();
        match parts.len() {
            1 => {
                let v = parts[0].parse()?;
                Ok(Borders::all(v))
            }
            2 => {
                let tb: Length = parts[0].parse()?;
                let lr: Length = parts[1].parse()?;
                Ok(Borders::each(tb.clone(), lr.clone(), tb, lr))
            }
            4 => {
                Ok(Borders::each(
                    parts[0].parse()?,
                    parts[1].parse()?,
                    parts[2].parse()?,
                    parts[3].parse()?))
            }
            _ => Err(Box::new(ParseStructError(format!(
                "Unexpected size format in {}, expects `<all>`, `<top-bottom>/<right-left>` or `<top>/<right>/<bottom>/<left>`",
                s
            ))))
        }
    }
}
impl fmt::Display for Borders {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.top, self.right, self.bottom, self.left
        )
    }
}

#[cfg(test)]
mod test {
    use crate::units::Borders;

    #[test]
    fn parse_1() {
        let str = "2cm";
        let borders: Borders = str.parse().unwrap();

        assert_eq!(borders.to_string(), "2cm/2cm/2cm/2cm");
    }

    #[test]
    fn parse_2() {
        let str = "1cm/2cm";
        let borders: Borders = str.parse().unwrap();

        assert_eq!(borders.to_string(), "1cm/2cm/1cm/2cm");
    }

    #[test]
    fn parse_4() {
        let str = "1cm/2cm/3cm/4cm";
        let borders: Borders = str.parse().unwrap();

        assert_eq!(borders.to_string(), str);
    }

    #[test]
    fn rotate() {
        let str = "1cm/2cm/3cm/4cm";
        let borders: Borders = str.parse().unwrap();
        let rot = borders.rotate_90();

        assert_eq!(rot.to_string(), "4cm/1cm/2cm/3cm");
    }
}
