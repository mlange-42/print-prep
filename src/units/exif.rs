use std::collections::HashMap;
lazy_static! {
    pub static ref ABBREVS: HashMap<&'static str, &'static str> = create_exif_abbreviations();
    pub static ref FIELDS: HashMap<&'static str, &'static str> =
        swap_map(create_exif_abbreviations());
}

fn swap_map(map: HashMap<&'static str, &'static str>) -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();

    for (k, v) in map.iter() {
        m.insert(*v, *k);
    }

    m
}
fn create_exif_abbreviations() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();

    m.insert("Mod", "Model");
    m.insert("SW", "Software");
    m.insert("A", "Artist");
    m.insert("F", "FocalLength");
    m.insert("Exp", "ExposureTime");
    m.insert("F/2", "FNumber");
    m.insert("Prog", "ExposureProgram");
    m.insert("ISO", "PhotographicSensitivity");
    m.insert("Date", "DateTimeOriginal");
    m.insert("Bias", "ExposureBiasValue");
    m.insert("MM", "MeteringMode");
    m.insert("EM", "ExposureMode");
    m.insert("LS", "LightSource");
    m.insert("CS", "ColorSpace");
    m.insert("SM", "SensingMethod");
    m.insert("WB", "WhiteBalance");

    m
}
