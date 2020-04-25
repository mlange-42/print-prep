//! Predefined exact formats

use crate::units::{Length, Size};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Converts format in cm to exact print formats in inches.
pub fn to_print_format(size: &Size) -> Result<Size, PrintFormatError> {
    let str = size.to_string();
    if size.width().is_none() || size.height().is_none() {
        return Err(PrintFormatError(format!(
            "Unable to determine print size. Missing dimension in size {}",
            &str
        )));
    }
    if FORMATS.contains_key::<str>(&str) {
        Ok(FORMATS.get::<str>(&str).unwrap().parse().unwrap())
    } else {
        Ok(size.clone())
    }
}

lazy_static! {
    pub static ref FORMATS: HashMap<&'static str, &'static str> = create_formats();
}

/// Error type for invalid print formats (missing dimensions).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintFormatError(String);

impl Error for PrintFormatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
impl fmt::Display for PrintFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

fn create_formats() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    // 9x13
    m.insert("13cm/9cm", "5in/3.5in");
    m.insert("9cm/13cm", "3.5in/5in");

    // 10x15
    m.insert("15cm/10cm", "6in/4in");
    m.insert("10cm/15cm", "4in/6in");

    // 13x18
    m.insert("18cm/13cm", "7in/5in");
    m.insert("13cm/18cm", "5in/7in");

    // 15x21
    m.insert("21cm/15cm", "8.5in/6in");
    m.insert("15cm/21cm", "6in/8.5in");

    // 18x24
    m.insert("24cm/18cm", "9.5in/7in");
    m.insert("18cm/24cm", "7in/9.5in");

    // TODO add more formats

    m
}

#[cfg(test)]
mod test {
    use crate::units::format::to_print_format;
    use crate::units::length::{Length, LengthUnit, ToLength};
    use crate::units::size::Size;

    #[test]
    fn print_formats() {
        let size: Size = "15cm/10cm".parse().unwrap();
        let format = to_print_format(&size).unwrap();

        assert_eq!(format.to_string(), "6in/4in".to_string());
    }
}
