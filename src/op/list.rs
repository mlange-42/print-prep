//! List files.

use crate::op::{ImageOperation, PathIterOperation};
use crate::util::PathUtil;
use path_absolutize::*;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

/// List files found by input pattern.
#[derive(StructOpt, Debug)]
pub struct ListFiles {
    /// Prints the full path.
    #[structopt(short, long)]
    pub path: bool,
    /// Prints the absolute path.
    #[structopt(short, long)]
    pub absolute: bool,
}

impl ImageOperation for ListFiles {
    fn execute(&self, files: &[PathBuf]) -> Result<(), Box<dyn Error>> {
        PathIterOperation::execute(self, &files)
    }
}

impl PathIterOperation for ListFiles {
    fn process_path(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let p = if self.path || self.absolute {
            if self.absolute {
                path.absolutize()
                    .ok()
                    .and_then(|p| Some(format!("{:?}", p)))
            } else {
                Some(format!("{:?}", path))
            }
        } else {
            PathUtil::name(&path)
        };
        if let Some(p) = p {
            println!("{}", p.replace("\\\\", "\\").replace("\"", ""));
        }
        Ok(())
    }
}
