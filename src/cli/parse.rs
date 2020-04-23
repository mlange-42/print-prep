use crate::ParseEnumError;
use image::imageops::FilterType;

pub fn parse_filter_type(str: &str) -> Result<FilterType, ParseEnumError> {
    match str {
        "nearest" => Ok(FilterType::Nearest),
        "linear" => Ok(FilterType::Triangle),
        "cubic" => Ok(FilterType::CatmullRom),
        "gauss" => Ok(FilterType::CatmullRom),
        "lanczos" => Ok(FilterType::Lanczos3),
        _ => Err(ParseEnumError(format!(
            "`{}` is not a valid filter type. Must be one of `(nearest|linear|cubic|gauss|lanczos)`",
            str
        ))),
    }
}
