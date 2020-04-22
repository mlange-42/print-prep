//! Path and file utilities

use std::path::PathBuf;

pub struct PathUtil {}

impl PathUtil {
    pub fn extension(path: &PathBuf) -> Option<String> {
        path.extension()
            .and_then(|ext| ext.to_str().and_then(|ext| Some(ext.to_lowercase())))
    }
}

#[cfg(test)]
mod test {
    use crate::util::PathUtil;
    use std::path::PathBuf;

    #[test]
    fn extension() {
        let path = PathBuf::from("C:/a/b/abc.jpg");
        let ext = PathUtil::extension(&path);

        assert_eq!(ext.unwrap(), "jpg")
    }
}
