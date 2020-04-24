//! Command-line interface for `print-prep`.

pub mod parse;

use crate::op::{ImageOperation, ScaleImage};
use std::error::Error;
use std::fmt;
use std::str::FromStr;
use structopt::StructOpt;

/// Command-line tool for photo print preparation and other bulk image operations.
///
/// Use `pprep -h`     for help, or
///     `pprep --help` for more detailed help, or
///     `pprep <subcommand> -h` for help on an operation.
///
/// For more documentation, see the GitHub repository:
///      https://mlange-42.github.io/print-prep/
#[derive(StructOpt, Debug)]
#[structopt(verbatim_doc_comment)]
pub struct Cli {
    /// List of input files or patterns. On Unix systems, patterns must be quoted!
    ///
    /// Examples:
    /// --input "path/to/*.jpg"
    /// --input "path/to/*.jpg" "other/path/to/*.jpg"
    /// --input image-0001.jpg image-0002.jpg image-0003.jpg
    ///
    #[structopt(verbatim_doc_comment)]
    #[structopt(short, long)]
    pub input: Vec<String>,

    /// Number of threads for parallel processing. Optional, default: number of processors.
    #[structopt(short, long)]
    pub threads: Option<usize>,

    /// Debug print parsed command line options.
    #[structopt(short, long)]
    pub debug: bool,

    /// Wait for user input after processing.
    #[structopt(short, long)]
    pub wait: bool,

    /// Input selection.
    #[structopt(subcommand)]
    pub op: Operation,
}

/// Image operations
#[allow(dead_code)]
#[derive(StructOpt, Debug)]
pub enum Operation {
    /// Scales images.
    Scale(ScaleImage),
}

impl Operation {
    /// Returns the associated ImageOperation.
    pub fn get_op(&self) -> &dyn ImageOperation {
        match self {
            Operation::Scale(sc) => sc,
        }
    }
}

impl FromStr for Cli {
    type Err = ParseCliError;

    /// Parses a string into a Cli.
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let quote_parts: Vec<_> = str.split('"').collect();
        let mut args: Vec<String> = vec![];
        for (i, part) in quote_parts.iter().enumerate() {
            let part = part.trim();
            if i % 2 == 0 {
                args.extend(
                    part.split(' ')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty()),
                );
            } else {
                args.push(part.to_string());
            }
        }
        Ok(Cli::from_iter(args.iter()))
    }
}

/// Error type for failed parsing command line argument.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCliError(String);

impl Error for ParseCliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for ParseCliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
